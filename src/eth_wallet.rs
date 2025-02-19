#![allow(unused)]
use anyhow::{bail,Result};

use secp256k1::{
    rand::{
        rngs,
        SeedableRng
    },
    PublicKey,
    SecretKey
};
use tiny_keccak::keccak256;
use web3::{
    transports::WebSocket,
    types::{Address, TransactionParameters, H256, U256},
    Web3,
};

use serde::{Serialize,Deserialize};
use std::io::{BufReader, BufWriter};
use std::fs::OpenOptions;
use std::str::FromStr;


pub fn gen_key_pair()->(SecretKey,PublicKey,){
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(111);
    secp.generate_keypair(&mut rng)
}

pub fn pub_key_addr(key:&PublicKey)->Address{
    let key = key.serialize_uncompressed();
    assert_eq!(key[0],0x04);
    let hash = keccak256(&key[1..]);
    Address::from_slice(&hash[12..])
}


pub async fn establish_web3_connection(url: &str) -> Result<Web3<WebSocket>> {
    let transport = web3::transports::WebSocket::new(url).await?;
    Ok(web3::Web3::new(transport))
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Wallet{
    pub_key : String,
    prv_key : String,
    addr : String
}
use hex;

impl Wallet{
    pub fn new(pub_key: &PublicKey,prv_key: &SecretKey)->Self{
        let addr = pub_key_addr(pub_key);
        Wallet{
            pub_key:pub_key.to_string(),
            prv_key:prv_key.to_string(),
            addr:format!("{:#x}", addr)
        }
    }
    
    pub fn write2file(&self,path:&str)->Result<()>{
        let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;
        
        // file.write()
        let  buff = BufWriter::new(file);
        serde_json::to_writer_pretty(buff, self)?;
        Ok(())
    }
    pub fn get_secret_key(&self) -> Result<SecretKey> {
        let key_bytes = hex::decode(&self.prv_key)  // Convert hex string to raw bytes
            .map_err(|_| anyhow::anyhow!("Failed to decode private key from hex"))?;
        let key = SecretKey::from_slice(&key_bytes)?;
        Ok(key)
    }
    
    pub fn get_pub_key(&self) -> Result<PublicKey> {
        let key_bytes = hex::decode(&self.pub_key)?;
        let key = PublicKey::from_slice(&key_bytes)?;
        Ok(key)
    }
    
    

    pub async fn get_balance(&self,conn: Web3<WebSocket>)->Result<U256>{
        let addr = Address::from_str(self.addr.as_str())?;
        let balance = conn.eth().balance(addr, None).await?;
        Ok(balance)
    }

}
use crate::utils;
pub fn create_eth_transaction(to: Address, eth_value: f64) -> TransactionParameters {
    TransactionParameters {
        to: Some(to),
        value: utils::eth_to_wei(eth_value),
        ..Default::default()
    }
}

pub async fn sign_and_send(
    web3: &Web3<web3::transports::WebSocket>,
    transaction: TransactionParameters,
    secret_key: &SecretKey,
) -> Result<H256> {
    let signed = web3
        .accounts()
        .sign_transaction(transaction, secret_key)
        .await?;

    let transaction_result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;
    Ok(transaction_result)
}

pub fn file2wallet(path:&str)->Result<Wallet>{
    let wallet;
    let file = 
    OpenOptions::new()
    .read(true)
    .open(path)?;
    let buff = BufReader::new(file);
    wallet = serde_json::from_reader(buff)?;
    Ok(wallet)
}

// pub fn write_to_file()