extern crate cita_web3;
extern crate cita_tool;
extern crate cita_types;
use std::str::FromStr;

use cita_tool::{
    client::basic::{Client, ClientExt},
    TransactionOptions, PrivateKey, crypto::Encryption, 
};

use cita_web3::web3::futures::Future;
use cita_types::U256;

fn main() {
    
    let url = {
        let mut args = ::std::env::args();
        args.nth(1).unwrap()
    };

    let (_eloop, transport) = cita_web3::web3::transports::Http::new(url.as_str()).unwrap();
    let web3 = cita_web3::web3::Web3::new(transport);
    let param = cita_web3::types::rpc_request::BlockNumberParams::new();


    let resp = web3
        .api::<cita_web3::api::Cita<cita_web3::web3::transports::Http>>()
        .call(param)
        .wait()
        .unwrap();
    println!("Response = {:?}", resp);


    
    let code: &str = "0x65766964656e63653d3078626161323361323662353066316365636633336266333730333532633432336430633930373663643835653061383939363038356466363535666539346666352674696d657374616d703d31303030";
    let address: &str = "0xffffffffffffffffffffffffffffffffff010000";
    // let value = U256::from_str("0");
    let encryption = Encryption::Secp256k1;
    let priv_key: PrivateKey = PrivateKey::from_str("< private key, without 0x >", encryption)
        .unwrap()
        .into();
    
    let tx_options = TransactionOptions::new()
        .set_code(code)
        .set_address(address)
        .set_value(Some(U256::from_str("0").unwrap()));        

    let client = Client::new();
    let mut client = client.set_uri(&url);
    let client = client.set_private_key(&priv_key);
    
    let a = client.send_raw_transaction(tx_options);
    println!("{:?} ", a);
}


    

