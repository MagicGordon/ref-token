use near_sdk_sim::{call, view, to_yocto};
use xref_token::ContractMetadata;
use near_sdk::json_types::U128;

mod common;
use crate::common::{
    init::*,
    utils::*
};

#[test]
fn test_stake(){
    // println!("ref_contract total: {}",
    //     view!(ref_contract.ft_total_supply()).unwrap_json::<U128>().0);

    // println!("user balance: {}",
    //     view!(ref_contract.ft_balance_of(user.valid_account_id())).unwrap_json::<U128>().0);

    // println!("ref_contract total: {}",
    //     view!(xref_contract.ft_total_supply()).unwrap_json::<U128>().0);

    let (_, _, user, ref_contract, xref_contract) = 
        init_env(true);

    call!(
        user,
        ref_contract.ft_transfer_call(xref_contract.valid_account_id(), to_yocto("10").into(), None, "".to_string()),
        deposit = 1
    )
    .assert_success();

    let current_xref_info = view!(xref_contract.contract_metadata()).unwrap_json::<ContractMetadata>();
    assert_xref(None, &current_xref_info, 0, to_yocto("10"), to_yocto("10"));
    assert_eq!(100000000_u128, view!(xref_contract.get_virtual_price()).unwrap_json::<U128>().0);
    assert_eq!(to_yocto("90"), view!(ref_contract.ft_balance_of(user.valid_account_id())).unwrap_json::<U128>().0);
    // call!(
    //     user,
    //     ref_contract.ft_transfer_call(xref_contract.valid_account_id(), to_yocto("10").into(), None, "".to_string()),
    //     deposit = 1
    // )
    // .assert_success();

    // let xref_info = view!(xref_contract.contract_metadata()).unwrap_json::<ContractMetadata>();
    // assert_xref(None, &xref_info, 0, to_yocto("10"), to_yocto("10"));
    // assert_eq!(100000000_u128, view!(xref_contract.get_virtual_price()).unwrap_json::<U128>().0);


    // call!(
    //     user,
    //     ref_contract.ft_transfer_call(xref_contract.valid_account_id(), to_yocto("10").into(), None, "reward".to_string()),
    //     deposit = 1
    // )
    // .assert_success();

    // call!(
    //     owner,
    //     xref_contract.modify_reward_per_sec(to_yocto("1").into())
    // )
    // .assert_success();
    
    // let xref_info = view!(xref_contract.contract_metadata()).unwrap_json::<ContractMetadata>();
    // println!("xref_info : {:?}", xref_info);


    // println!("----->> move to 1 secs later.");
    // assert!(root.borrow_runtime_mut().produce_block().is_ok());
    // println!("<<----- Chain goes 1 blocks, now #{}, ts:{}.", 
    // root.borrow_runtime().current_block().block_height, 
    // root.borrow_runtime().current_block().block_timestamp);

    // call!(
    //     user,
    //     ref_contract.ft_transfer_call(xref_contract.valid_account_id(), to_yocto("10").into(), None, "reward".to_string()),
    //     deposit = 1
    // )
    // .assert_success();

    // let xref_info = view!(xref_contract.contract_metadata()).unwrap_json::<ContractMetadata>();
    // println!("xref_info : {:?}", xref_info);


    // call!(
    //     root,
    //     ref_contract.storage_deposit(Some(user.valid_account_id()), None),
    //     deposit = to_yocto("1")
    // )
    // .assert_success();

}

#[test]
fn test_stake_no_register(){
    let (_, _, user, ref_contract, xref_contract) = 
    init_env(false);
    
    let out_come = call!(
        user,
        ref_contract.ft_transfer_call(xref_contract.valid_account_id(), to_yocto("10").into(), None, "".to_string()),
        deposit = 1
    );
    assert_eq!(get_error_count(&out_come), 1);
    assert!(get_error_status(&out_come).contains("Account not registered."));

    assert_eq!(to_yocto("100"), view!(ref_contract.ft_balance_of(user.valid_account_id())).unwrap_json::<U128>().0);

    let current_xref_info = view!(xref_contract.contract_metadata()).unwrap_json::<ContractMetadata>();
    assert_xref(None, &current_xref_info, 0, 0, 0);
}

#[test]
fn test_stake_zero(){
    let (_, _, user, ref_contract, xref_contract) = 
    init_env(true);
    
    let out_come = call!(
        user,
        ref_contract.ft_transfer_call(xref_contract.valid_account_id(), to_yocto("0").into(), None, "".to_string()),
        deposit = 1
    );
    assert_eq!(get_error_count(&out_come), 1);
    assert!(get_error_status(&out_come).contains("The amount should be a positive number"));

    assert_eq!(to_yocto("100"), view!(ref_contract.ft_balance_of(user.valid_account_id())).unwrap_json::<U128>().0);

    let current_xref_info = view!(xref_contract.contract_metadata()).unwrap_json::<ContractMetadata>();
    assert_xref(None, &current_xref_info, 0, 0, 0);
}