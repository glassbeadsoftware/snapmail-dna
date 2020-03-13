use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_persistence_api::{
        cas::content::Address
    },
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
};

/// Zome Function
/// Return list of all InMails that this agent did not acknowledge.
pub fn get_all_arrived_mail() -> ZomeApiResult<Vec<Address>> {
    // 1. Get all InMails with query
    let result = hdk::query("inmail".into(),
                            0, 0)?;
    // For each InMail
    let mut unreads = Vec::new();
    for inmail_address in &result {
        //   2. Get Acknowledgment private link
        let res_count = hdk::get_links_count(inmail_address, "acknowledgment".into(), LinkMatch::Any)?;
        //      b. if true continue
        if res.count > 0 {
            continue;
        }
        //   3. Add to result list
        unreads.push(inmail_address.clone());
    }
    Ok(unreads)
}