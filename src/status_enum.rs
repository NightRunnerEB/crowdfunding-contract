use multiversx_sc::derive_imports::*;

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy, Debug)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}