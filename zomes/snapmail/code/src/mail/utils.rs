// use hdk::prelude::*;

use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
        time::Timeout,
    },
};
use holochain_wasm_utils::{
    api_serialization::get_entry::{
        GetEntryOptions, StatusRequestKind, GetEntryResultType,
    },
};

use crate::{
    mail::entries::*,
    AgentAddress,
};

/// Conditions: Must be a single author entry type
pub(crate) fn get_entry_and_author(address: &Address) -> ZomeApiResult<(AgentAddress, Entry)> {
    let get_options = GetEntryOptions {
        status_request: StatusRequestKind::Latest,
        entry: true,
        headers: true,
        timeout: Timeout::default(),
    };
    let maybe_entry_result = hdk::get_entry_result(address, get_options);
    if let Err(err) = maybe_entry_result {
        hdk::debug(format!("Failed getting address: {}", err)).ok();
        return Err(err);
    }
    let entry_result = maybe_entry_result.unwrap();
    let entry_item = match entry_result.result {
        GetEntryResultType::Single(item) => {
            item
        },
        _ => panic!("Asked for latest so should get Single"),
    };
    assert!(entry_item.headers.len() > 0);
    assert!(entry_item.headers[0].provenances().len() > 0);
    let author = entry_item.headers[0].provenances()[0].source();
    let entry = entry_item.entry.expect("Should have Entry");
    Ok((author, entry))
}

pub(crate) fn get_pending_mail(pending_address: &Address) -> ZomeApiResult<(AgentAddress, PendingMail)> {
    let (author, entry) = get_entry_and_author(pending_address)?;
    let pending = crate::into_typed::<PendingMail>(entry).expect("Should be PendingMail");
    Ok((author, pending))
}

pub(crate) fn get_pending_ack(ack_address: &Address) -> ZomeApiResult<(AgentAddress, PendingAck)> {
    let (author, entry) = get_entry_and_author(ack_address)?;
    let ack = crate::into_typed::<PendingAck>(entry).expect("Should be PendingAck");
    Ok((author, ack))
}

/// Return address of created InAck
pub(crate) fn create_and_commit_inack(outmail_address: &Address, from: &AgentAddress) -> ZomeApiResult<Address> {
    // Create InAck
    let inack = InAck::new();
    let inack_entry = Entry::App("inack".into(), inack.into());
    let inack_address = hdk::commit_entry(&inack_entry)?;
    let json_from = serde_json::to_string(from).expect("Should stringify");
    // Create link from OutMail
    let _ = hdk::link_entries(
        outmail_address,
        &inack_address,
        "receipt",
        json_from.as_str().into(),
    )?;
    Ok(inack_address)
}
