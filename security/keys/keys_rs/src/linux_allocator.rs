use alloc::heap::{Alloc, AllocErr, Layout};
use os;
use c_types;

pub struct LinuxAllocator {}

unsafe impl<'a> Alloc for &'a LinuxAllocator {
	// TODO obey alignment???

	unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
		// TODO unfortunately have to emulate kmalloc here as it's always inlined....
		// __kmalloc does not support large allocations, punts to kmalloc_large
		// and a bunch of __always_inlined__ functions and macros.
		// if (layout.size() > os::KMALLOC_MAX_CACHE_SIZE) {
		// 	Err(AllocErr::Unsupported("Asked for large allocation, fix this!!!!!!!"))
		// }
		let ptr = os::krealloc(0 as *const c_types::c_void, layout.size(), 0x90) as *mut c_types::c_uchar; // 0x90 = GFP_KERNEL
		if ptr.is_null() {
			return Err(AllocErr::Exhausted{request:layout}); // TODO not Exhausted
		}
		return Ok(ptr);
	}

	unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout) {
		os::kfree(ptr as *const c_types::c_void);
	}
}