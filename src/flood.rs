use substrate_api_client::{
    substrate,
};
use serde_json::json;

#[tokio::main]
async fn flood() {
    // Connect to a Substrate node
    let api = substrate().await;

    // Get the metadata for the chain
    let metadata = api.metadata().await.unwrap();

    // Find the pallet that we want to call an extrinsic for
    let pallet_name = "pallet_example";
    let pallet = metadata
        .pallets
        .iter()
        .find(|p| p.name == pallet_name)
        .unwrap();

    // Find the extrinsic we want to call
    let extrinsic_name = "function_example";
    let extrinsic = pallet
        .calls
        .iter()
        .find(|e| e.name == extrinsic_name)
        .unwrap();

    // Build the arguments for the extrinsic
    let arguments = json!([1, 2, 3]);

    // Number of times to submit the extrinsic
    let num_submissions = 10;

    for i in 0..num_submissions {
        // Get the latest block number and block hash
        let block_number = api.chain.get_block_number().await.unwrap();
        let block_hash = api.chain.get_block_hash(block_number).await.unwrap();

        // Submit the extrinsic to the node
        let result = api
            .runtime
            .submit_extrinsic(extrinsic.call, block_hash, arguments)
            .await
            .unwrap();
        println!("Submission {}: Extrinsic submitted: {:?}", i + 1, result);
    }
}
