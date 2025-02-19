
#![allow(unused)]

use web3::types::U256;


pub fn wei2eth(wei:U256)->f64{
    wei.as_u128() as f64 / 1.0e18
}

use std::time::{UNIX_EPOCH,SystemTime};

pub fn get_ns()->u64{
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    dur.as_secs()<<30 | dur.subsec_nanos() as u64
}
pub fn eth_to_wei(eth_val: f64) -> U256 {
    let result = eth_val * 1_000_000_000_000_000_000.0;
    let result = result as u128;

    U256::from(result)
}