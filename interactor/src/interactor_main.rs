#![allow(non_snake_case)]

mod proxy;
mod interactor_cli;
mod interactor_state;

use interactor_cli::*;
use interactor_state::State;
use clap::Parser;
use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk;

const GATEWAY: &str = sdk::gateway::DEVNET_GATEWAY;

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
        Commands::Claim { address} => {
            interact.claim(address).await;
        }
    }
}

struct ContractInteract {
    interactor: Interactor,
    owner_address: Bech32Address,
    wallet_address: Bech32Address,
    contract_code: BytesValue,
    state: State,
}

impl ContractInteract {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let owner_address = interactor.register_wallet(Wallet::from_pem_file("owner.pem").unwrap());

        // PASSWORD: "alice"
        // InsertPassword::Plaintext("alice".to_string()) || InsertPassword::StandardInput
        let wallet_address = interactor.register_wallet(
            Wallet::from_keystore_secret(
                "alice.json",
                InsertPassword::Plaintext("alice".to_string()),
            )
            .unwrap(),
        );

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/crowdfunding.mxsc.json",
            &InterpreterContext::default(),
        );

        Self {
            interactor,
            owner_address: owner_address.into(),
            wallet_address: wallet_address.into(),
            contract_code: contract_code,
            state: State::load_state(),
        }
    }

    async fn deploy(&mut self, target: u128, deadline: u64) {
        let target_biguint = BigUint::<StaticApi>::from(target);

        let new_address = self
            .interactor
            .tx()
            .from(&self.owner_address)
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

        println!("Owner address: {new_address_bech32}");
    }

    async fn fund(&mut self, amount: u128) {
        let egld_amount = BigUint::<StaticApi>::from(amount);
        let alice_address = self.wallet_address.to_string();
        println!("Alice address: {alice_address}");

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

    async fn claim(&mut self, address: &str) {
        let claim_address = Bech32Address::from_bech32_string(address.to_string());

        let response = self
            .interactor
            .tx()
            .from(claim_address)
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
