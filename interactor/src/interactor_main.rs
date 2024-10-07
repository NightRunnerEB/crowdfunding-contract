#![allow(non_snake_case)]

mod proxy;

use clap::{Parser, Subcommand};
use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};


const GATEWAY: &str = sdk::gateway::DEVNET_GATEWAY;
const STATE_FILE: &str = "state.toml";

#[derive(Parser)]
#[command(name = "ContractInteract")]
#[command(about = "CLI for interacting with the crowdfunding contract", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    Deploy {
        /// Target value (as BigUint)
        #[arg(short, long)]
        target: u128,

        /// Deadline value (as u64)
        #[arg(short, long)]
        deadline: u64,
    },

    Fund {
        /// Amount of EGLD to fund (as BigUint)
        #[arg(short, long)]
        amount: u128,
    },

    GetTarget,

    GetDeadline,

    GetDeposit {
        /// Bech32 address of the donor
        #[arg(short, long)]
        donor: String,
    },

    /// Check the current status of the contract
    Status,

    /// Get the current funds in the contract
    GetCurrentFunds,

    /// Claim funds from the contract
    Claim,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut interact = ContractInteract::new().await;

    match &cli.command {
        Commands::Deploy { target, deadline } => {
            interact.deploy(*target, *deadline).await;
        }
        Commands::Fund { amount } => {
            interact.fund(*amount).await;
        }
        Commands::GetTarget => {
            interact.target().await;
        }
        Commands::GetDeadline => {
            interact.deadline().await;
        }
        Commands::GetDeposit { donor } => {
            interact.deposit(donor).await;
        }
        Commands::Status => {
            interact.status().await;
        }
        Commands::GetCurrentFunds => {
            interact.get_current_funds().await;
        }
        Commands::Claim => {
            interact.claim().await;
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    contract_address: Option<Bech32Address>,
}

impl State {
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State,
}

impl ContractInteract {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(test_wallets::alice());

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/crowdfunding.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state(),
        }
    }

    async fn deploy(&mut self, target: u128, deadline: u64) {
        let target_biguint = BigUint::<StaticApi>::from(target);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::CrowdfundingProxy)
            .init(target_biguint, deadline)
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;

        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

        println!("new address: {new_address_bech32}");
    }

    async fn fund(&mut self, amount: u128) {
        let egld_amount = BigUint::<StaticApi>::from(amount);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::CrowdfundingProxy)
            .fund()
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn target(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::CrowdfundingProxy)
            .target()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn deadline(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::CrowdfundingProxy)
            .deadline()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn deposit(&mut self, donor: &str) {
        let donor_address = Bech32Address::from_bech32_string(donor.to_string());

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::CrowdfundingProxy)
            .deposit(donor_address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn status(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::CrowdfundingProxy)
            .status()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn get_current_funds(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::CrowdfundingProxy)
            .get_current_funds()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn claim(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::CrowdfundingProxy)
            .claim()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }
}
