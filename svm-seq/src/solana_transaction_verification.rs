use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signature};
use solana_transaction_status::{EncodedTransaction, UiMessage, UiTransactionEncoding};
use std::str::FromStr;
use solana_sdk::{instruction::CompiledInstruction, message::Message, pubkey::Pubkey, hash::Hash};
use solana_sdk::signature::Signer;

fn main() {
    // Initialize RPC client
    let node_client = RpcClient::new("https://api.testnet.solana.com");

    // Example signature
    let str_sign = "3z1bip6CS4mWPoFb97DbPjNUqiQSt8KnHmAX92B3WLix7sAttb9MTbrru8B5T94AfBfW7rrAdtJ9aNJWzKsPct8o";
    let signatures = Signature::from_str(str_sign).unwrap();
    // println!("Signature: {:?}", signatures);


    // Retrieve transaction details
    let get_txn = node_client.get_transaction(
        &signatures,
        UiTransactionEncoding::Json,
    ).unwrap();
    println!("Transaction: {:?}", get_txn.transaction.transaction);

    // Map the transaction
    match get_txn.transaction.transaction {
        EncodedTransaction::Json(ui_transaction) => {
            match ui_transaction.message {
                UiMessage::Raw(raw_msg) => {
                    // compile
                    let mut ins_test: Vec<CompiledInstruction> = Vec::new();
                    for instruction in raw_msg.instructions {
                        let compiled_instruction = CompiledInstruction {
                            program_id_index: instruction.program_id_index,
                            accounts: instruction.accounts.clone(),
                            data: bs58::decode(&instruction.data).into_vec().unwrap(),
                        };
                        ins_test.push(compiled_instruction); // Add to the vector
                    }

                    // account pub key
                    let accountKey: Vec<Pubkey> = raw_msg.account_keys.iter().map(|x| Pubkey::from_str(x).unwrap()).collect();
                    let acc_byte = accountKey[0].to_bytes();

                    // Construct Serialize message
                    let message = Message::new_with_compiled_instructions(
                        raw_msg.header.num_required_signatures,
                        raw_msg.header.num_readonly_signed_accounts,
                        raw_msg.header.num_readonly_unsigned_accounts,
                        accountKey.clone(),
                        Hash::from_str(&raw_msg.recent_blockhash).unwrap(),
                        ins_test,
                    ).serialize();

                    // Verify signature
                    let okko = signatures.verify(&acc_byte, &message);
                    println!("Signature Verification: {:?}", okko);
                }
                _ => {
                    println!("Transaction MSG is not in JSON format");
                }
            }
        }
        _ => {
            println!("Transaction is not in JSON format");
        }
    }
}
