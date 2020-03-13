use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
        agent::AgentId,
        time::Timeout,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError
    },
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
    api_serialization::get_entry::{
        GetEntryOptions, StatusRequestKind, GetEntryResultType,
    },
};

use crate::{
    mail::entries::*,
    AgentAddress, DirectMessageProtocol, MailMessage, AckMessage, ReceivedMail,
};

/// Conditions: Must be a single author entry type
pub(crate) fn get_entry_and_author(address: &Address) -> ZomeApiResult<(AgentAddress, Entry)> {
    let get_options = GetEntryOptions {
        status_request: StatusRequestKind::Latest,
        entry: true,
        headers: true,
        timeout: Timeout::default(),
    };
    let maybe_entry_result = hdk::get_entry_result(pending_address, get_options);
    if let Err(err) = maybe_entry_result {
        hdk::debug("Failed getting address:");
        hdk::debug(&err);
        return Err(err);
    }
    let entry_result = maybe_entry_result.unwrap();
    let entry_item = match entry_result.result {
        GetEntryResultType::Single(item) => {
            item
        },
        _ => panic!("Asked for latest so should get Single"),
    };
    assert!(entry_item.headers.size() > 0);
    assert!(entry_item.headers[0].provenances()[0] > 0);
    let author = entry_item.headers[0].provenances()[0].source();
    Ok((author, pending))
}

pub(crate) fn get_pending_mail(pending_address: &Address) -> ZomeApiResult<(AgentAddress, PendingMail)> {
    let (author, entry) = get_entry_and_author(pending_address)?;
    let pending = crate::into_typed::<PendingMail>(entry.unwrap()).expect("Should be PendingMail");
    Ok((author, pending))
}

pub(crate) fn get_pending_ack(ack_address: &Address) -> ZomeApiResult<(AgentAddress, PendingAck)> {
    let (author, entry) = get_entry_and_author(ack_address)?;
    let ack = crate::into_typed::<PendingAck>(entry.unwrap()).expect("Should be AckReceiptEncrypted");
    Ok((author, ack))
}

/// Return address of created InAck
pub(crate) fn create_and_commit_inack(outmail_address: &Address, from: &AgentAddress) -> ZomeApiResult<Address> {
    // Create InAck
    let inack = InAck::new();
    let inack_entry = Entry::App("inack".into(), inack.into());
    let inack_address = hdk::commit_entry(&inack_entry)?;
    // Create link from OutMail
    let _ = hdk::link_entries(
        outmail_address,
        &inack_address,
        "receipt",
        from.into(),
    )?;
    Ok(inack_address)
}
