#![allow(unexpected_cfgs)]
#![cfg_attr(target_os = "solana", feature(const_mut_refs))]

use allogator::Allogator;
use solana_pubkey::Pubkey;

use std::alloc::Layout;

pub const ID: Pubkey = solana_pubkey::pubkey!("gatorgatorgatorgatorgatorgatorgatorgatorgat");
pub const SCRATCH: Pubkey = solana_pubkey::pubkey!("scratchscratchscratchscratchscratchscratchs");

/// The layout required for a len 4 vector of type u64
#[allow(unused)]
const VEC_U64_LAYOUT: Layout = unsafe {
    Layout::from_size_align_unchecked(
        8 * core::mem::size_of::<u64>(),
        core::mem::align_of::<u64>(),
    )
};

#[allow(unused)]
static ALLOGATOR: (Allogator, &[usize]) = const {
    let mut allogator = Allogator::new();
    let ptr_1 = allogator.const_allocate(VEC_U64_LAYOUT);
    let ptr_2 = allogator.const_allocate(VEC_U64_LAYOUT);

    (allogator, &[ptr_1, ptr_2])
};

#[cfg(target_os = "solana")]
#[global_allocator]
static A: Allogator = ALLOGATOR.0;

#[no_mangle]
pub unsafe extern "C" fn entrypoint(_input: *mut u8) -> u32 {
    // SAFETY:
    // 1) Preallocated
    // 2) 0 length
    // 3) Correct capacity
    solana_program::msg!("using const allocation");
    let vec_ptr = ALLOGATOR.1[0];
    let mut compile_allo_vec = Vec::from_raw_parts(vec_ptr as *mut u64, 0, 8);
    compile_allo_vec.push(2);
    compile_allo_vec.push(3);
    compile_allo_vec.push(4);
    compile_allo_vec.push(5);
    compile_allo_vec.push(2);
    compile_allo_vec.push(3);
    compile_allo_vec.push(4);
    compile_allo_vec.push(5);

    // vec.push(8) // this will reallocate and use cus (gobble gobble?)

    // This is a runtime allocation!
    solana_program::msg!("using runtime allocation");
    let mut runtime_allo_vec = Vec::with_capacity(8);
    runtime_allo_vec.push(2);
    runtime_allo_vec.push(3);
    runtime_allo_vec.push(4);
    runtime_allo_vec.push(5);
    runtime_allo_vec.push(2);
    runtime_allo_vec.push(3);
    runtime_allo_vec.push(4);
    runtime_allo_vec.push(5);
    std::hint::black_box((compile_allo_vec, runtime_allo_vec));

    solana_program::msg!("done");

    0
}
