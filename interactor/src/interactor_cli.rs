use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ContractInteract")]
#[command(about = "CLI for interacting with the crowdfunding contract", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {

    Deploy {
        #[arg(short, long)]
        target: u128,

        #[arg(short, long)]
        deadline: u64,
    },

    Fund {
        #[arg(short, long)]
        amount: u128,
    },

    GetTarget,

    GetDeadline,

    GetDeposit {
        #[arg(short, long)]
        donor: String,
    },

    Status,

    GetCurrentFunds,

    Claim{
        #[arg(short, long)]
        address: String,
    },
}