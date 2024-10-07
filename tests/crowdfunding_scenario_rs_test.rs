use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/crowdfunding.mxsc.json", crowdfunding::ContractBuilder);
    blockchain
}

#[test]
fn crowdfunding_fund_rs() {
    world().run("scenarios/crowdfunding-fund.scen.json");
}

#[test]
fn crowdfunding_init_rs() {
    world().run("scenarios/crowdfunding-init.scen.json");
}

#[test]
fn test_fund_too_late_rs() {
    world().run("scenarios/test-fund-too-late.scen.json");
}
