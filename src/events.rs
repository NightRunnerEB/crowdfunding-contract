
#[multiversx_sc::module]
pub trait CrowdfundingEvents {
    #[event("fundingReceived")]
    fn funding_received_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] amount: BigUint,
    );

    #[event("claimedFunds")]
    fn claimed_funds_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] amount: BigUint,
    );

    #[event("fundsRefunded")]
    fn funds_refunded_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] refunded_amount: BigUint,
    );

    #[event("contractInitialized")]
    fn contract_initialized_event(
        &self,
        #[indexed] owner: ManagedAddress,
        #[indexed] target: BigUint,
        #[indexed] deadline: u64,
    );
}
