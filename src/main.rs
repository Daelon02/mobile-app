#[macro_use]
extern crate clap;

use clap::App;

use sp_core::sr25519;

use node_template_runtime::{Block, Header, SignedBlock};
use std::sync::mpsc::channel;
use substrate_api_client::Api;

fn main() {
    env_logger::init();
    let url = "rpc.realis.network";
    // if no signer is set in the whole program, we need to give to Api a specific type instead of an associated type
    // as during compilation the type needs to be defined.
    let api = Api::<sr25519::Pair>::new(format!("wss://{}", url)).unwrap();

    let head = api.get_finalized_head().unwrap().unwrap();

    println!("Finalized Head:\n {} \n", head);

    let h: Header = api.get_header(Some(head)).unwrap().unwrap();
    println!("Finalized header:\n {:?} \n", h);

    let b: SignedBlock = api.get_signed_block(Some(head)).unwrap().unwrap();
    println!("Finalized signed block:\n {:?} \n", b);

    println!(
        "Latest Header: \n {:?} \n",
        api.get_header::<Header>(None).unwrap()
    );

    println!(
        "Latest block: \n {:?} \n",
        api.get_block::<Block>(None).unwrap()
    );

    println!("Subscribing to finalized heads");
    let (sender, receiver) = channel();
    api.subscribe_finalized_heads(sender).unwrap();

    for _ in 0..5 {
        let head: Header = receiver
            .recv()
            .map(|header| serde_json::from_str(&header).unwrap())
            .unwrap();
        println!("Got new Block {:?}", head);
    }
}
