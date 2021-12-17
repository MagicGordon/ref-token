use near_sdk_sim::{call, view, to_yocto};
use xref_token::ContractMetadata;
use near_sdk::json_types::U128;

mod common;
use crate::common::{
    init::*,
    utils::*
};

#[test]
fn test_reward(){
    let (root, owner, user, ref_contract, xref_contract) = 
        init_env(true);
    let mut total_reward = 0;
    let mut total_locked = 0;
    let mut total_supply = 0;
    call!(
        user,
        ref_contract.ft_transfer_call(xref_contract.valid_account_id(), to_yocto("10").into(), None, "".to_string()),
        deposit = 1
    )
    .assert_success();
    total_locked += to_yocto("10");
    total_supply += to_yocto("10");

    let xref_info1 = view!(xref_contract.contract_metadata()).unwrap_json::<ContractMetadata>();
    assert_xref(None, &xref_info1, 0, total_locked, total_supply);
    assert_eq!(100000000_u128, view!(xref_contract.get_virtual_price()).unwrap_json::<U128>().0);
    assert_eq!(to_yocto("90"), view!(ref_contract.ft_balance_of(user.valid_account_id())).unwrap_json::<U128>().0);

    call!(
        owner,
        xref_contract.modify_reward_per_sec(to_yocto("1").into())
    )
    .assert_success();

    call!(
        owner,
        ref_contract.ft_transfer_call(xref_contract.valid_account_id(), to_yocto("100").into(), None, "reward".to_string()),
        deposit = 1
    )
    .assert_success();
    total_reward += to_yocto("100");

    assert!(root.borrow_runtime_mut().produce_block().is_ok());

    let xref_info2 = view!(xref_contract.contract_metadata()).unwrap_json::<ContractMetadata>();
    let time_diff = nano_to_sec(xref_info2.prev_distribution_time) - nano_to_sec(xref_info1.prev_distribution_time);
    println!("time_diff : {}", time_diff);
    total_reward -= time_diff as u128 * xref_info1.reward_per_sec.0;
    total_locked += time_diff as u128 * xref_info1.reward_per_sec.0;
    assert_xref(Some(xref_info1), &xref_info2, total_reward, total_locked, total_supply);
    assert_eq!(xref_info2.locked_token_amount.0 * 100_000_000 / xref_info2.supply.0, view!(xref_contract.get_virtual_price()).unwrap_json::<U128>().0);

    

}