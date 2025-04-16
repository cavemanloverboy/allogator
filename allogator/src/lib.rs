#![allow(unexpected_cfgs)]
#![cfg_attr(target_os = "solana", feature(const_mut_refs))]
//! Allogator: An SVM BumpAllocator that enables compile time allocations

#[derive(Debug, Clone, Copy)]
pub struct Allogator {
    pub start: usize,
    /// This is initialized to full 32KB heap length, but is truncated
    /// at compile time to exclude any compile time allocations.
    ///
    /// This could actually be removed but is kept for legacy reasons
    /// (aka i am lazy)
    pub len: usize,
    /// Cursor during compile time allocations
    pub pos: usize,
}

/// Start address of the memory region used for program heap.
pub const HEAP_START_ADDRESS: usize = 0x300000000;
/// Length of the heap memory region used for program heap.
pub const HEAP_LENGTH: usize = 32 * 1024;

impl Allogator {
    pub const fn new() -> Allogator {
        Allogator {
            start: HEAP_START_ADDRESS,
            len: HEAP_LENGTH,
            pos: HEAP_START_ADDRESS + HEAP_LENGTH,
        }
    }
    pub const fn const_allocate(&mut self, layout: std::alloc::Layout) -> usize {
        // Subtracts size after rounding up to type alignment
        //
        // Note: this (default behavior) is actually kinda dumb bc if you have align > 8 type,
        // you will needlessly waste heap to align. Not a big problem since most ppl have
        // <=16 aligned types and don't use most of the heap.
        self.pos = self.pos.saturating_sub(layout.size());
        self.pos &= !(layout.align().wrapping_sub(1));
        self.len = self.len.saturating_sub(layout.size());
        self.len &= !(layout.align().wrapping_sub(1));

        // We've gone past start! Out of memory!
        if self.pos < self.start + core::mem::size_of::<*mut u8>() {
            return 0;
        }
        self.pos
    }
}
/// Integer arithmetic in this global allocator implementation is safe when
/// operating on the prescribed `HEAP_START_ADDRESS` and `HEAP_LENGTH`. Any
/// other use may overflow and is thus unsupported and at one's own risk.
#[allow(clippy::arithmetic_side_effects)]
unsafe impl std::alloc::GlobalAlloc for Allogator {
    // Bump Allocator
    // [8 byte position usize][heap bytes except for 8 bytes]
    //
    // Allogator
    // [8 byte position][runtime heap][compile time allocations]
    #[inline]
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let pos_ptr = self.start as *mut usize;

        let mut pos = unsafe { *pos_ptr };
        if pos == 0 {
            // First time, set starting position to end of non-compile time used heap
            pos = self.pos;
        }

        // Subtracts size after rounding up to type alignment
        //
        // Note: this (default behavior) is actually kinda dumb bc if you have align > 8 type,
        // you will needlessly waste heap to align. Not a big problem since most ppl have
        // <=16 aligned types and don't use most of the heap.
        pos = pos.saturating_sub(layout.size());
        pos &= !(layout.align().wrapping_sub(1));

        // We've gone past start! Out of memory!
        if pos < self.start + std::mem::size_of::<*mut u8>() {
            return core::ptr::null_mut();
        }

        // Update cursor with current position and return
        unsafe {
            *pos_ptr = pos;
        }
        pos as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, _: *mut u8, _: std::alloc::Layout) {
        // I'm a bump allocator, I don't free
    }
}
