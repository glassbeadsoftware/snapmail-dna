// use hdk::prelude::*;

use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::cas::content::Address,
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
    holochain_persistence_api::hash::HashString,
};
use crate::{
    AgentAddress,
    mail::entries::OutMail,
};

/// Zome function
/// Check if agent received receipts from all receipients of one of its OutMail.
/// If false, returns list of agents who's receipt is missing.
pub fn has_mail_been_received(outmail_address: Address) -> ZomeApiResult<Result<(), Vec<AgentAddress>>> {
    // 1. get OutMail
    let outmail = hdk::utils::get_as_type::<OutMail>(outmail_address.clone())?;
    // 2. Merge all recepients lists into one
    let all_recepients: Vec<AgentAddress> = [outmail.mail.to, outmail.mail.cc, outmail.bcc].concat();
    hdk::debug(format!("all_recepients: {:?}", all_recepients)).ok();
    // 3. get all ``receipt`` links
    let links_result = hdk::get_links(&outmail_address, LinkMatch::Exactly("receipt"), LinkMatch::Any)?;
    hdk::debug(format!("links_result: {:?}", links_result)).ok();
    // 4. Make list of Receipt authors
    let receipt_authors: Vec<AgentAddress> = links_result.tags()
        .iter().map(|from_str| {
        let hashstr: String = serde_json::from_str(from_str).unwrap();
        HashString::from(hashstr)
    })
        .collect();
    hdk::debug(format!("receipt_authors: {:?}", receipt_authors)).ok();

    // 5. Diff lists
    let diff: Vec<AgentAddress>  = all_recepients.into_iter()
        .filter(|recepient| !receipt_authors.contains(recepient))
        .collect();
    hdk::debug(format!("diff: {:?}", diff)).ok();
    Ok(if diff.len() > 0 {
        Err(diff)
    } else {
        Ok(())
    })
}
