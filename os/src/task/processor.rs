//!Implementation of [`Processor`] and Intersection of control flow
//!
//! Here, the continuous operation of user apps in CPU is maintained,
//! the current running state of CPU is recorded,
//! and the replacement and transfer of control flow of different applications are executed.

use super::__switch;
use super::{fetch_task, TaskStatus};
use super::{TaskContext, TaskControlBlock};
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use crate::config::{BIG_STRIDE, PAGE_SIZE};
use crate::mm::{VirtAddr, VirtPageNum, MapPermission};
use alloc::sync::Arc;
use lazy_static::*;

/// Processor management structure
pub struct Processor {
    ///The task currently executing on the current processor
    current: Option<Arc<TaskControlBlock>>,

    ///The basic control flow of each core, helping to select and switch process
    idle_task_cx: TaskContext,
}

impl Processor {
    ///Create an empty Processor
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }

    ///Get mutable reference to `idle_task_cx`
    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }

    ///Get current task in moving semanteme
    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    ///Get current task in cloning semanteme
    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }

    fn mmap(&self, start: usize, len: usize, port: usize) -> isize {
        if start & (PAGE_SIZE - 1) != 0 {
            println!("mmap failed: start address is not page-aligned");
            return -1;
        }

        if port > 7usize || port == 0 {
            println!("mmap failed: invalid port number");
            return -1;
        }

        let memory_set = &mut self.current.as_ref().unwrap().inner_exclusive_access().memory_set;

        let start_vaddr = VirtAddr(start);
        let end_vaddr = VirtAddr(start + len);
        let start_vpn = VirtPageNum::from(start_vaddr);
        let end_vpn = VirtPageNum::from(end_vaddr.ceil());

        for vpn in start_vpn.0 .. end_vpn.0 {
            if let Some(pte) = memory_set.translate(VirtPageNum(vpn)) {
                if pte.is_valid() {
                    println!("mmap failed: address already mapped");
                    return -1;
                }
            }
        }

        let permission = MapPermission::from_bits((port as u8) << 1).unwrap()| MapPermission::U;
        memory_set.insert_framed_area(start_vaddr, end_vaddr, permission);
        0
    }

    fn munmap(&self, start: usize, len: usize) -> isize {
        if start & (PAGE_SIZE - 1) != 0 {
            println!("munmap failed: start address is not page-aligned");
            return -1;
        }

        let memory_set = &mut self.current.as_ref().unwrap().inner_exclusive_access().memory_set;

        let start_vaddr = VirtAddr(start);
        let end_vaddr = VirtAddr(start + len);
        let start_vpn = VirtPageNum::from(start_vaddr);
        let end_vpn = VirtPageNum::from(end_vaddr.ceil());

        for vpn in start_vpn.0 .. end_vpn.0 {
            if let Some(pte) = memory_set.translate(VirtPageNum(vpn)) {
                if !pte.is_valid() {
                    println!("munmap failed: address not mapped");
                    return -1;
                }
            }
        }

        memory_set.unmap(start_vaddr, end_vaddr);
        0
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}

/// mmap
pub fn mmap(start: usize, len: usize, port: usize) -> isize {
    PROCESSOR.exclusive_access().mmap(start, len, port)
}

/// munmap
pub fn munmap(start: usize, len: usize) -> isize {
    PROCESSOR.exclusive_access().munmap(start, len)
}

///The main part of process execution and scheduling
///Loop `fetch_task` to get the process that needs to run, and switch the process through `__switch`
pub fn run_tasks() {
    loop {
        let mut processor = PROCESSOR.exclusive_access();
        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
            // access coming task TCB exclusively
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;

            let pass = BIG_STRIDE / task_inner.get_priority();
            let task_stride = task_inner.get_stride();
            task_inner.stride = task_stride.wrapping_sub(pass);
            // release coming task_inner manually
            drop(task_inner);
            // release coming task TCB manually
            processor.current = Some(task);
            // release processor manually
            drop(processor);
            unsafe {
                __switch(idle_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            warn!("no tasks available in run_tasks");
        }
    }
}

/// Get current task through take, leaving a None in its place
pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().take_current()
}

/// Get a copy of the current task
pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().current()
}

/// Get the current user token(addr of page table)
pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    task.get_user_token()
}

///Get the mutable reference to trap context of current task
pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}

///Return to idle control flow for new scheduling
pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.exclusive_access();
    let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
    drop(processor);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr);
    }
}
