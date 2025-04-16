use litesvm::LiteSVM;
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_message::Message;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;

fn scratch_account() -> Account {
    // Account Metas = 3 * 16 = 48 bytes
    let first_account_key_address: usize = 0x400000010;
    let first_account_writable: u8 = 1;
    let first_account_signer: u8 = 1;
    let first_pad = [0_u8; 6];
    let second_account_key_address: usize = 0x400005220;
    let second_account_writable: u8 = 1;
    let second_account_signer: u8 = 0;
    let second_pad = [0_u8; 6];
    let third_account_key_address: usize = first_account_key_address;
    let third_account_writable: u8 = first_account_writable;
    let third_account_signer: u8 = first_account_signer;
    let third_pad = [0_u8; 6];

    // Instruction data is 120 bytes
    let mut instruction_data = [0; 120];
    instruction_data[0] = 3;
    // instruction_data[4..36].copy_from_slice(self.base.unwrap_or(self.from).key());
    instruction_data[36..44].copy_from_slice(&u64::to_le_bytes("counter-seed".len() as u64));

    let offset = 44 + "counter-seed".len();
    instruction_data[44..offset].copy_from_slice("counter-seed".as_bytes());
    instruction_data[offset..offset + 8].copy_from_slice(&946560_u64.to_le_bytes());
    instruction_data[offset + 8..offset + 16].copy_from_slice(&8_u64.to_le_bytes());
    instruction_data[offset + 16..offset + 48].copy_from_slice(scratch::ID.as_ref());

    // account infos: 56 * 3
    let info_pad = [0_u8; 5];
    let c_account_1_key = first_account_key_address;
    let c_account_1_lamports: usize = first_account_key_address + 64;
    let c_account_1_data_len: usize = 0; // maybe change in future
    let c_account_1_data_ptr = first_account_key_address + 32 + 32 + 8 + 8;
    let c_account_1_owner_ptr = first_account_key_address + 32;
    let c_account_1_rent_epoch = u64::MAX;
    let c_account_1_is_signer = first_account_signer;
    let c_account_1_is_writable = first_account_writable;
    let c_account_1_is_executable = 0;
    // 2
    let c_account_2_key = second_account_key_address;
    let c_account_2_lamports = second_account_key_address + 64;
    let c_account_2_data_len: usize = 0; // maybe change in future
    let c_account_2_data_ptr = second_account_key_address + 32 + 32 + 8 + 8;
    let c_account_2_owner_ptr = second_account_key_address + 32;
    let c_account_2_rent_epoch = u64::MAX;
    let c_account_2_is_signer = second_account_signer;
    let c_account_2_is_writable = second_account_writable;
    let c_account_2_is_executable = 0;
    // 3
    let c_account_3_key = third_account_key_address;
    let c_account_3_lamports = third_account_key_address + 64;
    let c_account_3_data_len: usize = 0; // maybe change in future
    let c_account_3_data_ptr = third_account_key_address + 32 + 32 + 8 + 8;
    let c_account_3_owner_ptr = third_account_key_address + 32;
    let c_account_3_rent_epoch = u64::MAX;
    let c_account_3_is_signer = third_account_signer;
    let c_account_3_is_writable = third_account_writable;
    let c_account_3_is_executable = 0;

    let mut scratch_account_data = Vec::with_capacity(48 + 120 + 56 * 3);
    scratch_account_data.extend_from_slice(&first_account_key_address.to_le_bytes());
    scratch_account_data.extend_from_slice(&[first_account_writable]);
    scratch_account_data.extend_from_slice(&[first_account_signer]);
    scratch_account_data.extend_from_slice(&first_pad);
    scratch_account_data.extend_from_slice(&second_account_key_address.to_le_bytes());
    scratch_account_data.extend_from_slice(&[second_account_writable]);
    scratch_account_data.extend_from_slice(&[second_account_signer]);
    scratch_account_data.extend_from_slice(&second_pad);
    scratch_account_data.extend_from_slice(&third_account_key_address.to_le_bytes());
    scratch_account_data.extend_from_slice(&[third_account_writable]);
    scratch_account_data.extend_from_slice(&[third_account_signer]);
    scratch_account_data.extend_from_slice(&third_pad);
    //ix
    scratch_account_data.extend_from_slice(&instruction_data);
    //infos
    scratch_account_data.extend_from_slice(&c_account_1_key.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_1_lamports.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_1_data_len.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_1_data_ptr.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_1_owner_ptr.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_1_rent_epoch.to_le_bytes());
    scratch_account_data.extend_from_slice(&[c_account_1_is_signer]);
    scratch_account_data.extend_from_slice(&[c_account_1_is_writable]);
    scratch_account_data.extend_from_slice(&[c_account_1_is_executable]);
    scratch_account_data.extend_from_slice(&info_pad);
    scratch_account_data.extend_from_slice(&c_account_2_key.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_2_lamports.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_2_data_len.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_2_data_ptr.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_2_owner_ptr.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_2_rent_epoch.to_le_bytes());
    scratch_account_data.extend_from_slice(&[c_account_2_is_signer]);
    scratch_account_data.extend_from_slice(&[c_account_2_is_writable]);
    scratch_account_data.extend_from_slice(&[c_account_2_is_executable]);
    scratch_account_data.extend_from_slice(&info_pad);
    scratch_account_data.extend_from_slice(&c_account_3_key.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_3_lamports.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_3_data_len.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_3_data_ptr.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_3_owner_ptr.to_le_bytes());
    scratch_account_data.extend_from_slice(&c_account_3_rent_epoch.to_le_bytes());
    scratch_account_data.extend_from_slice(&[c_account_3_is_signer]);
    scratch_account_data.extend_from_slice(&[c_account_3_is_writable]);
    scratch_account_data.extend_from_slice(&[c_account_3_is_executable]);

    Account {
        lamports: LAMPORTS_PER_SOL,
        data: scratch_account_data,
        owner: scratch::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}

fn main() {
    let mut svm = LiteSVM::new()
        .with_blockhash_check(false)
        .with_sigverify(false)
        .with_transaction_history(0);
    svm.add_program_from_file(scratch::ID, "../target/deploy/scratch.so")
        .unwrap();
    let payer = Keypair::new();
    let payer_key = payer.pubkey();
    svm.airdrop(&payer_key, 100 * LAMPORTS_PER_SOL).unwrap();

    let scratch_key = solana_pubkey::pubkey!("scratchscratchscratchscratchscratchscratchs");
    svm.set_account(scratch_key, scratch_account()).unwrap();

    let counter_key = Pubkey::create_with_seed(&payer_key, "counter-seed", &scratch::ID).unwrap();
    let instruction = Instruction {
        program_id: scratch::ID,
        accounts: vec![
            AccountMeta::new(payer_key, true),
            AccountMeta::new(scratch_key, false),
            AccountMeta::new(counter_key, false),
            AccountMeta::new_readonly(Pubkey::default(), false),
        ],
        data: vec![],
    };
    let message = Message::new(&[instruction], Some(&payer_key));

    let transaction = Transaction::new_unsigned(message);

    let res = svm.send_transaction(transaction);

    match res {
        Ok(res) => {
            // println!("{}", res.pretty_logs());
            for log in res.logs {
                println!("    {log}");
            }
        }
        Err(e) => {
            // println!("{}", e.meta.pretty_logs());
            for log in e.meta.logs {
                println!("    {log}");
            }
        }
    }
}
