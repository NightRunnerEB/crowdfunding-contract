#![no_std]

multiversx_sc::imports!();
pub mod events;

use multiversx_sc::derive_imports::*;

#[multiversx_sc::contract]
pub trait Crowdfunding: events::CrowdfundingEvents {

    #[view(getTarget)]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    #[view(getDeadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, target: &BigUint, deadline: u64) {
        self.target().set(target);
        self.deadline().set(&deadline);

        let owner = self.blockchain().get_owner_address();
        self.contract_initialized_event(owner.clone(), target.clone(), deadline);
    }

    #[endpoint]
    #[payable("EGLD")]
    fn fund(&self) {
        let payment = self.call_value().egld_value();

        require!(
            self.status() == Status::FundingPeriod,
            "cannot fund after deadline"
        );

        let caller = self.blockchain().get_caller();
        self.deposit(&caller).update(|deposit| *deposit += &*payment);

        self.funding_received_event(caller.clone(), payment.clone_value());
    }

    #[view]
    fn status(&self) -> Status {
        if self.blockchain().get_block_timestamp() <= self.deadline().get() {
            Status::FundingPeriod
        } else if self.get_current_funds() >= self.target().get() {
            Status::Successful
        } else {
            Status::Failed
        }
    }
  
    #[view(getCurrentFunds)]
    fn get_current_funds(&self) -> BigUint {
        self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0)
    }

    #[endpoint]
    fn claim(&self) {
        match self.status() {
            Status::FundingPeriod => sc_panic!("cannot claim before deadline"),
            Status::Successful => {
                let caller = self.blockchain().get_caller();
                require!(
                    caller == self.blockchain().get_owner_address(),
                    "only owner can claim successful funding"
                );

                let sc_balance = self.get_current_funds();
                self.send().direct_egld(&caller, &sc_balance);

                self.claimed_funds_event(caller.clone(), sc_balance.clone());
            },
            Status::Failed => {
                let caller = self.blockchain().get_caller();
                let deposit = self.deposit(&caller).get();
                if deposit > 0u32 {
                    self.deposit(&caller).clear();
                    self.send().direct_egld(&caller, &deposit);

                    self.funds_refunded_event(caller.clone(), deposit.clone());
                }
            }
        }
    }
}

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy, Debug)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}
