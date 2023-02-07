use substrate_api_client::{
    substrate,
};
use serde_json::json;

#[tokio::main]
async fn main() {
    // Connect to a Substrate node
    let api = substrate().await;

    // The address of the account whose balance we want to query
    let address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

    // Build the query parameters
    let params = json!([address, "Balances", "FreeBalance"]);

    // Send the query to the `pallet_balances` pallet
    let result = api
        .state
        .query_storage("pallet_balances", "FreeBalance", params)
        .await
        .unwrap();

    // Extract the balance from the result
    let balance = hex::decode(result.substring(2..)).unwrap();
    let balance = u128::from_le_bytes(balance[..].try_into().unwrap());

    println!("Balance of {}: {}", address, balance);
}
