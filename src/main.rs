use clap::Parser;
#[derive(Parser)]
#[command(name = "Simple Bitcoin Transaction Decoder")]
#[command(version = "0.21")]
#[command(about = "Decodes any Bitcoin transaction given in hex format")]
struct Cli {
    #[arg(required = true, help = "(string, required) Raw transaction hex")]
    transaction_hex: String,
}

fn main() {
    let cli = Cli::parse();
    match transaction_decoder::decode(cli.transaction_hex) {
        Ok(json) => println!("{}", json),
        Err(err) => eprintln!("{}", err),
    }
}
