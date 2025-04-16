#![allow(unexpected_cfgs)]
#![cfg_attr(target_os = "solana", feature(const_mut_refs))]

use allogator::Allogator;
use solana_pubkey::Pubkey;

use std::alloc::Layout;

pub const ID: Pubkey = solana_pubkey::pubkey!("irwonirwonirwonirwonirwonirwonirwonirwonirw");
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

use pinocchio::{
    instruction::{Account, AccountMeta},
    lazy_entrypoint::InstructionContext,
    syscalls::sol_invoke_signed_c,
};

#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u32 {
    let mut ctx = InstructionContext::new(input);

    // Load accounts (run around and paniccc if not >=3)
    let payer = ctx.next_account().unwrap().assume_account();
    let scratch = ctx.next_account().unwrap().assume_account();
    let account = ctx.next_account().unwrap().assume_account();

    // Verifications to use scratch
    // (payer must have zero data for hardcoded addresses to work)
    // (if you want to use nonzero data you have to update these addresses as well
    //  which is doable via your_account.key() as *const [u8; 32])
    assert!(payer.data_is_empty());
    assert!(scratch.key() == SCRATCH.as_ref());

    // Now update the data we must update in scratch
    let scratch_data_ptr = unsafe { scratch.borrow_mut_data_unchecked().as_mut_ptr() };
    *scratch_data_ptr.add(52).cast::<[u8; 32]>() = *payer.key();

    let metas_ptr = scratch_data_ptr;
    let instruction_data_ptr = scratch_data_ptr.add(48);

    // Although, as we discovered on stream, account infos can't be on stack, this
    // *might* be able to go on stack which would save another 10-50-ish cus (
    // rough approximation obviously)
    let instruction = CInstruction {
        program_id: &[0; 32],
        data: instruction_data_ptr,
        data_len: 120,
        accounts: metas_ptr.cast_const().cast(),
        accounts_len: 3,
    };

    let infos: [Account; 3] = [
        Account::from(&payer),
        Account::from(&account),
        Account::from(&payer),
    ];

    unsafe {
        sol_invoke_signed_c(
            &instruction as *const CInstruction as *const u8,
            infos.as_ptr() as *const u8,
            3,
            core::ptr::null(/* this is ok only bc we are passing 0 seeds */),
            0,
        );
    }

    0
}

#[derive(Debug)]
#[repr(C)]
pub struct CAccount {
    // Public key of the account.
    key: *const Pubkey,

    // Number of lamports owned by this account.
    lamports: *const u64,

    // Length of data in bytes.
    data_len: u64,

    // On-chain data within this account.
    data: *const u8,

    // Program that owns this account.
    owner: *const Pubkey,

    // The epoch at which this account will next owe rent.
    rent_epoch: u64,

    // Transaction was signed by this account's key?
    is_signer: bool,

    // Is the account writable?
    is_writable: bool,

    // This account's data contains a loaded program (and is now read-only).
    executable: bool,
}

#[derive(Debug)]
#[repr(C)]

struct CInstruction<'a> {
    /// Public key of the program.
    program_id: *const pinocchio::pubkey::Pubkey,

    /// Accounts expected by the program instruction.
    accounts: *const AccountMeta<'a>,

    /// Number of accounts expected by the program instruction.
    accounts_len: u64,

    /// Data expected by the program instruction.
    data: *const u8,

    /// Length of the data expected by the program instruction.
    data_len: u64,
}
