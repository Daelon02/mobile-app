use sp_core::sr25519;
use serde_json::*;
use std::sync::mpsc::channel;
use substrate_api_client::Api;
use sp_runtime::*;

fn main() {
    env_logger::init();
    let url = "rpc.realis.network";
    // if no signer is set in the whole program, we need to give to Api a specific type instead of an associated type
    // as during compilation the type needs to be defined.
    let api = Api::<sr25519::Pair>::new(format!("wss://{}", url)).unwrap();

    let head = api.get_finalized_head().unwrap().unwrap();

    println!("Finalized Head:\n {} \n", head);
}
