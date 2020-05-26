extern crate cita_web3;
//extern crate web3;

use cita_web3::web3::futures::Future;
//use web3::futures::Future;

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
}
