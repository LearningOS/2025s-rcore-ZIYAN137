//! Process management syscalls
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, get_syscall_times, mmap, munmap};
use crate::mm::{VirtAddr, virt_to_phys, get_flags, PTEFlags};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");

    let vaddr = VirtAddr(ts as usize);
    if let Ok(paddr) = virt_to_phys(vaddr) {
        let time_us = get_time_us();
        let ts = paddr.0 as *mut TimeVal;
        unsafe {
            *ts = TimeVal{
                sec: time_us / 1_000_000,
                usec: time_us % 1_000_000,
            }
        }
        0
    } else {
        -1
    }
}

pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            let vaddr = VirtAddr(id);
            if let Ok(pte_flags) = get_flags(vaddr) {
                if !pte_flags.contains(PTEFlags::R | PTEFlags::U) {
                    return -1;
                }
                if let Ok(id) = virt_to_phys(vaddr) {
                    let id = id.0 as *const u8;
                    unsafe {
                        return *id as isize
                    }
                }
            }
            -1
        }
        1 => {
            let vaddr = VirtAddr(id);
            if let Ok(pte_flags) = get_flags(vaddr) {
                if !pte_flags.contains(PTEFlags::W | PTEFlags::U) {
                    return -1;
                }
                if let Ok(id) = virt_to_phys(vaddr) {
                    let id = id.0 as *mut u8;
                    unsafe {
                        *id = data as u8;
                    }
                    return 0
                }
            }
            -1
        }
        2 => {
            let syscall_times = get_syscall_times();
            syscall_times[id] as isize
        }
        _ => -1
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    mmap(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    munmap(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
