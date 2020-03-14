//use hdk::prelude::*;

use hdk::{
    error::{ZomeApiResult},
    holochain_persistence_api::{
        cas::content::Address
    },
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
};
use crate::mail;

/// Return list of outMail addresses for which we succesfully linked a new InAck
pub fn check_incoming_ack() -> ZomeApiResult<Vec<Address>> {
    // Lookup `ack_inbox` links on my agentId
    let links_result = hdk::get_links(
        &*hdk::AGENT_ADDRESS,
        LinkMatch::Exactly("ack_inbox"),
        LinkMatch::Any)?;
    // For each link
    let mut updated_outmails = Vec::new();
    for pending_ack_address in &links_result.addresses() {
        //  - Get entry on the DHT
        let maybe_pending_ack = mail::get_pending_ack(pending_ack_address);
        if let Err(err) = maybe_pending_ack {
            continue;
        }
        let (author, pending_ack) = maybe_pending_ack.unwrap();
        // Create InAck
        let maybe_inack_address = mail::create_and_commit_inack(&pending_ack.outmail_address, &author);
        if let Err(err) = maybe_inack_address {
            hdk::debug(format!("Creating InAck from PendignAck failed: {}", err));
            continue;
        }
        //  - Delete link from my agentId
        let res = hdk::remove_link(
            *hdk::AGENT_ADDRESS,
            &pending_ack_address,
            "ack_inbox",
            "",
        );
        if let Err(err) = res {
            hdk::debug("Remove ``ack_inbox`` link failed:");
            hdk::debug(err);
            continue;
        }
        // Delete PendingAck
        let res = hdk::remove_entry(pending_ack_address);
        if let Err(err) = res {
            hdk::debug(format!("Delete PendignAck failed: {}", err));
        }
        // Add to return list
        updated_outmails.push(pending_ack.outmail_address.clone());
    }
    Ok(updated_outmails)
}