use serde::{Deserialize, Serialize};

use super::dt::ZRpcDt;

#[macro_export]
macro_rules! params {
    ($($a:expr),*) => {{
        use libzrpc::types::dt::ZRpcDtAuto;

        let mut res: Vec<ZRpcDt> = vec![];
        $(
            res.push($a.to_zdt());
        )*
        res
    }};
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZRpcReq(pub String, pub Vec<ZRpcDt>);

impl ZRpcReq {
    pub fn new(proc: &str, params: Vec<ZRpcDt>) -> Self {
        Self(proc.to_string(), params)
    }
}
