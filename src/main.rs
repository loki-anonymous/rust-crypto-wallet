use anyhow::Result;
mod eth_wallet;
mod utils;
use std::env;
use std::str::FromStr;
use web3::types::Address;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let (secret_key, pub_key) = eth_wallet::gen_key_pair();

    println!("secret key: {}", &secret_key.to_string());
    println!("public key: {}", &pub_key.to_string());

    let pub_address = eth_wallet::pub_key_addr(&pub_key);
    println!("public address: {:?}", pub_address);

    let crypto_wallet = eth_wallet::Wallet::new( &pub_key,&secret_key);
    println!("crypto_wallet: {:?}", &crypto_wallet);

    let wallet_file_path = "crypto_wallet.json";
    crypto_wallet.write2file(wallet_file_path)?;

    let loaded_wallet = eth_wallet::file2wallet(wallet_file_path)?;
    println!("loaded_wallet: {:?}", loaded_wallet);

    let endpoint = env::var("INFURA_RINKEBY_WS")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let block_number = web3_con.eth().block_number().await?;
    println!("block number: {}", &block_number);
    
    println!("balance {:?}",loaded_wallet.get_balance(web3_con.clone()).await);
    let transaction = eth_wallet::create_eth_transaction(Address::from_str("0xC93f068dF0d25f89A45Ff10d066e1Db20a5a56b0")?, 0.01);

    let t_hash = eth_wallet::sign_and_send(&web3_con, transaction, &loaded_wallet.get_secret_key()?).await?;
    println!("{:?}",t_hash);

    Ok(())
}