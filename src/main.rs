use clap::Parser;
use ethers::providers::{Http, Provider};
use ethers::{
    providers::Middleware,
    signers::{LocalWallet, Signer},
    utils::hex,
};

use ethers::types::Address as EthAddress;
use solana_sdk::signature::Signer as SolSigner;
use solana_sdk::{bs58, signature::Keypair};
use solana_sdk::client::Client as SOLClient;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};


#[derive(Parser, Debug)]
#[command(name = "address-gen")]
#[command(about = "Batch Address Generation CLI Tool")]
#[command(version = "0.1.0")]
struct Cli {
    /// Number of addresses to generate
    #[arg(short, long)]
    count: Option<usize>,

    /// Network type
    #[arg(short, long)]
    network: Option<String>,

    /// Output file path
    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(long)]
    version: bool,
}

async fn calculate_eth_total_balance(
    file_path: &str,
    provider_url: &str,
) -> Result<f64, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(provider_url)?;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut total_balance = 0.0;

    for line in reader.lines().skip(1) {
        let line = line?;
        let parts: Vec<&str> = line.split(",").collect();
        if parts.len() >= 3 {
            let address = parts[2];
            let eth_address = address.parse::<EthAddress>()?;
            let balance = provider.get_balance(eth_address, None).await?;
            let balance_eth = ethers::utils::format_units(balance, "ether")?;
            total_balance += balance_eth.parse::<f64>()?;
        }
    }
    Ok(total_balance)
}

// async fn calculate_sol_total_balance(file_path: &str,rpc_url: &str) -> Result<f64> {
//     // let client = SOLClient::tpu_addr(&self).getr?
//     let file = File::open(file_path)?;
//     let reader = BufReader::new(file);
    
//     let mut total_balance = 0.0;
// }

fn generate_sol_addresses(count: usize) -> Vec<(String, String)> {
    (0..count)
        .map(|_| {
            let key_pair = Keypair::new();
            let pubkey = key_pair.try_pubkey().unwrap().to_string();
            let private_key = bs58::encode(key_pair.to_bytes()).into_string();
            (pubkey, private_key)
        })
        .collect()
}

fn generator_evm_addresses(count: usize) -> Vec<(String, String)> {
    (0..count)
        .map(|_| {
            let wallet = LocalWallet::new(&mut rand::thread_rng());

            let address = wallet.address();

            let private_key = wallet.signer().to_bytes();
            let private_key_hex = hex::encode(private_key);

            (format!("0x{:x}", address).to_lowercase(), private_key_hex)
        })
        .collect()
}

fn save_to_csv(addresses: &[(String, String)], path: &PathBuf) -> Result<(), anyhow::Error> {
    let file = File::create(path)?;
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&["Index", "Network", "Public Address", "Private Key"])?;
    for (index, (pubkey, private_key)) in addresses.iter().enumerate() {
        wtr.write_record(&[&(index + 1).to_string(), "EVM", pubkey, private_key])?;
    }

    wtr.flush()?;

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    if cli.version {
        println!("v0.1.0");
        std::process::exit(0);
    }

    let count = match cli.count {
        Some(c) if c > 0 => c,
        _ => {
            println!("Error: Please specify a number greater than 0 using -c or --count");
            println!("Use --help to view help information");
            std::process::exit(1);
        }
    };
    let network = cli.network.unwrap_or_else(|| "evm".to_string());

    println!(
        "🚀 Generating {} {} addresses...",
        count,
        network.to_uppercase()
    );

    let addresses = match network.to_lowercase().as_str() {
        "evm" => generator_evm_addresses(count),
        "solana" => generate_sol_addresses(count),
        _ => return Err(anyhow::anyhow!("Unsupported network type")),
    };

    match cli.output {
        Some(output) => {
            save_to_csv(&addresses, &output)?;
            println!("✅ Address generation completed, saved to {:?}", output);
        }
        None => {
            for (index, (pubkey, private_key)) in addresses.iter().enumerate() {
                println!(
                    "Index: {}, Public Address: {}, Private Key: {}",
                    index + 1,
                    pubkey,
                    private_key
                );
            }
            println!("✅ Address generation address completed");
        }
    }

    Ok(())
}
