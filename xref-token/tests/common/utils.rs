use xref_token::ContractMetadata;
use near_sdk_sim::ExecutionResult;

pub fn nano_to_sec(nano: u64) -> u64 {
    nano / 1_000_000_000
}

pub fn assert_xref(
    before_xref: Option<ContractMetadata>,
    current_xref: &ContractMetadata,
    undistribute_reward: u128,
    locked_token_amount: u128,
    supply: u128,
    
) {
    if let Some(xref) = before_xref{
        let reward = xref.reward_per_sec.0 * (nano_to_sec(current_xref.prev_distribution_time) - nano_to_sec(xref.prev_distribution_time)) as u128;
        assert_eq!(current_xref.undistribute_reward.0, 
            if undistribute_reward != 0 {undistribute_reward} else {xref.undistribute_reward.0 - reward});
        assert_eq!(current_xref.locked_token_amount.0, 
            if locked_token_amount != 0 {locked_token_amount} else {xref.locked_token_amount.0 + reward});
        assert_eq!(current_xref.supply.0, supply);
    }else{
        assert_eq!(current_xref.undistribute_reward.0, undistribute_reward);
        assert_eq!(current_xref.locked_token_amount.0, locked_token_amount);
        assert_eq!(current_xref.supply.0, supply);
    }
    
}

pub fn get_error_count(r: &ExecutionResult) -> u32 {
    r.promise_errors().len() as u32
}

pub fn get_error_status(r: &ExecutionResult) -> String {
    format!("{:?}", r.promise_errors()[0].as_ref().unwrap().status())
}