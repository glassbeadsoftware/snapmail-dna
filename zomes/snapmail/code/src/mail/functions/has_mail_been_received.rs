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
pub fn has_mail_been_received(outmail_address: &Address) -> ZomeApiResult<Result<(), Vec<&AgentAddress>>> {
    // 1. get OutMail
    let outmail = hdk::utils::get_as_type::<OutMail>(outmail_address.clone())?;
    // 2. Merge all recepients lists into one
    let all_recepients: Vec<AgentAddress> = [outmail.mail.to, outmail.mail.cc, outmail.bcc].concat();
    // 3. get all ``receipt`` links
    let links_result = hdk::get_links(outmail_address, LinkMatch::Exactly("receipt"), LinkMatch::Any)?;
    // 4. Make list of Receipt authors
    let receipt_authors: Vec<AgentAddress> = links_result.tags()
        .iter().map(|from_str| HashString::from(*from_str))
        .collect();
    // 5. Diff lists
    let diff: Vec<&AgentAddress>  = all_recepients.iter().filter(|recepient| !receipt_authors.contains(*recepient)).collect();
    Ok(if diff.len() > 0 {
        Err(diff)
    } else {
        Ok(())
    })
}
