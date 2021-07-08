use substrate_api_client::{Api, node_metadata};
use substrate_api_client::sp_runtime::app_crypto::sp_core::sr25519;

fn main() {
    // instantiate an Api that connects to the given address
    let url = "rpc.realis.network";
    // if no signer is set in the whole program, we need to give to Api a specific type instead of an associated type
    // as during compilation the type needs to be defined.
    let api = Api::<sr25519::Pair>::new(format!("wss://{}", url)).unwrap();

    let meta = Api::get_metadata(&api).unwrap();
    println!("Metadata:\n {}", node_metadata::Metadata::pretty_format(&meta).unwrap());
}
