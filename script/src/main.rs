use mpl_token_metadata::accounts::Metadata;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

fn main() {
    let rpc = RpcClient::new("https://api.testnet.solana.com");
    let pubkey_str = "PUBKEY_OF_DATA_ACCOUNT";

    match Pubkey::from_str(pubkey_str) {
        Ok(pubkey) => match rpc.get_account(&pubkey) {
            Ok(account) => match Metadata::from_bytes(&account.data) {
                Ok(meta) => {
                    println!("Sol balance of {} is {:?}", pubkey_str, meta);
                }
                Err(e) => {
                    eprintln!("Failed to parse metadata: {:?}", e);
                    println!("Raw account data: {:?}", account.data);
                }
            },
            Err(e) => {
                eprintln!("Failed to get account: {:?} 11", e);
            }
        },
        Err(e) => {
            eprintln!("Failed to parse public key: {:?}", e);
        }
    }
}
