use solana_rpc_client::rpc_client::RpcClient;
use solana_transaction_status::UiTransactionEncoding;

fn main() {
    let node_client = RpcClient::new("https://api.devnet.solana.com");
    let get_txn = node_client.get_transaction(
        &Signature::from_str(
            &String::from(
                "4cyZRzDngW1wxTB9hmgCG9oq7Q96iCBGF1QX3UhdM7n1WDxBmFdKntgiG6Di1p6BVBRm63cnBZNbt1tnf95oXk52"
            )
        ).unwrap(),
        UiTransactionEncoding::JsonParsed,
    ).unwrap();

    let transaction_result = serde_json::to_string(&get_txn)
        .expect("Failed to serialize the struct to JSON");

    let transaction_result_json: NewRoot = serde_json::from_str(&transaction_result).unwrap();
    println!("{:?}", transaction_result_json);
}