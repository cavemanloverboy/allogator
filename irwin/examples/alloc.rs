use litesvm::LiteSVM;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_signer::Signer;
use solana_transaction::Transaction;

fn main() {
    let mut svm = LiteSVM::new()
        .with_blockhash_check(false)
        .with_sigverify(false)
        .with_transaction_history(0);
    svm.add_program_from_file(irwin::ID, "../target/deploy/irwin.so")
        .unwrap();
    let payer = Keypair::new();
    let payer_key = payer.pubkey();
    svm.airdrop(&payer_key, 100 * LAMPORTS_PER_SOL).unwrap();

    let instruction = Instruction {
        program_id: irwin::ID,
        accounts: vec![],
        data: vec![],
    };
    let message = Message::new(&[instruction], Some(&payer_key));

    let transaction = Transaction::new_unsigned(message);

    let res = svm.send_transaction(transaction).unwrap();

    for log in res.logs {
        println!("    {log}");
    }
}
