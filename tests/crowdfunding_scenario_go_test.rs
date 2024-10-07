use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn crowdfunding_fund_go() {
    world().run("scenarios/crowdfunding-fund.scen.json");
}

#[test]
fn crowdfunding_init_go() {
    world().run("scenarios/crowdfunding-init.scen.json");
}

#[test]
fn test_fund_too_late_go() {
    world().run("scenarios/test-fund-too-late.scen.json");
}
