use anyhow::Result;
mod eth_wallet;
mod utils;
use std::env;
use std::str::FromStr;
use web3::types::Address;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    // let (secret_key, pub_key) = eth_wallet::gen_key_pair();

    // println!("secret key: {}", &secret_key.to_string());
    // println!("public key: {}", &pub_key.to_string());
    let sender_private_key = env::var("SENDER_PRIVATE_KEY")?;
    let receiver_address = env::var("RECEIVER_ADDRESS")?;

    // let pub_address = eth_wallet::pub_key_addr(&pub_key);
    // println!("public address: {:?}", pub_address);

    // let crypto_wallet = eth_wallet::Wallet::new( &pub_key,&secret_key);
    // println!("crypto_wallet: {:?}", &crypto_wallet);

    // let wallet_file_path = "crypto_wallet.json";
    // crypto_wallet.write2file(wallet_file_path)?;

    // let loaded_wallet = eth_wallet::file2wallet(wallet_file_path)?;
    // println!("loaded_wallet: {:?}", loaded_wallet);

    let endpoint = env::var("INFURA_RINKEBY_WS")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    // let block_number = web3_con.eth().block_number().await?;
    // println!("block number: {}", &block_number);

    // let loaded_wallet = eth_wallet::Wallet::from_private_key(&sender_private_key)?;
    // println!("Loaded wallet: {:?}", loaded_wallet);

    // println!("balance {:?}",loaded_wallet.get_balance(web3_con.clone()).await);
    let transaction = eth_wallet::create_eth_transaction(Address::from_str(&receiver_address)?, 0.01);

    let t_hash = eth_wallet::sign_and_send(&web3_con, transaction, &eth_wallet::get_secret_key_from_str(&sender_private_key)?).await?;
    println!("{:?}",t_hash);
    
    let receipt = eth_wallet::get_recipt(&web3_con, t_hash).await;
    println!("Transaction receipt: {:?}", receipt);

    Ok(())
}