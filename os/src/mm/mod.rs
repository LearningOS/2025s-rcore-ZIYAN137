//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{kernel_stack_position, MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};
pub use page_table::{PTEFlags, PageTable};
use crate::task::current_user_token;

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

/// translate VirtAddr to PhysAddr
pub fn virt_to_phys(vaddr: VirtAddr) -> Result<PhysAddr, &'static str> {
    let offset = vaddr.page_offset();
    let vpn = vaddr.floor();
    let ppn = PageTable::from_token(current_user_token()).translate(vpn)
        .map(|pte| pte.ppn());
    if let Some(ppn) = ppn {
        return Ok(PhysAddr::from(usize::from(PhysAddr::from(ppn)) | offset))
    } else {
        Err("virt_to_phys failed")
    }
}

/// get PTE flags
pub fn get_flags(vaddr: VirtAddr) -> Result<PTEFlags, &'static str> {
    let vpn = vaddr.floor();
    if let Some(pte) = PageTable::from_token(current_user_token()).translate(vpn) {
        return Ok(pte.flags())
    } else {
        Err("get_flags failed")
    }
}