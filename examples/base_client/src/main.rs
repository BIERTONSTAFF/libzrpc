use std::net::Ipv4Addr;

use libzrpc::{client::ZRpcClient, params, types::dt::ZRpcDt};

#[tokio::main]
async fn main() {
    let mut client = ZRpcClient::new((Ipv4Addr::LOCALHOST, 3000)).await.unwrap();

    match client.call("add", params!("SECRET_KEY", 2, 2)).await {
        Ok(ZRpcDt::Int32(res)) => println!("Sum: {}", res),
        Err(e) => eprintln!("{}", e),
        _ => {}
    }
}
