// use hdk::prelude::*;

use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
    },
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
};
use crate::{
    link_kind, entry_kind,
    mail::{self, entries::InMail},
};

/// Return list of new InMail addresses
pub fn check_incoming_mail() -> ZomeApiResult<Vec<Address>> {
    let maybe_my_handle_address = crate::handle::get_my_handle_entry();
    if let None = maybe_my_handle_address {
        return Err(ZomeApiError::Internal("This agent does not have a Handle set up".to_string()));
    }
    let my_handle_address = maybe_my_handle_address.unwrap().0;
    // Lookup `mail_inbox` links on my agentId
    let links_result = hdk::get_links(
        // &*hdk::AGENT_ADDRESS,
        &my_handle_address,
        LinkMatch::Exactly(link_kind::MailInbox),
        LinkMatch::Any,
    )?;
    hdk::debug(format!("incoming_mail links_result: {:?} (for {})", links_result, &my_handle_address)).ok();
    // For each link
    let mut new_inmails = Vec::new();
    for pending_address in &links_result.addresses() {
        //  1. Get entry on the DHT
        hdk::debug(format!("pending mail address: {}", pending_address)).ok();
        let maybe_pending_mail = mail::get_pending_mail(pending_address);
        if let Err(err) = maybe_pending_mail {
            hdk::debug(format!("Getting PendingMail from DHT failed: {}", err)).ok();
            continue;
        }
        let (author, pending) = maybe_pending_mail.unwrap();
        //  2. Convert and Commit as InMail
        let inmail = InMail::from_pending(pending, author);
        let inmail_entry = Entry::App(entry_kind::InMail.into(), inmail.into());
        let maybe_inmail_address = hdk::commit_entry(&inmail_entry);
        if maybe_inmail_address.is_err() {
            hdk::debug("Failed committing InMail").ok();
            continue;
        }
        new_inmails.push(maybe_inmail_address.unwrap());
        //  3. Remove link from this agentId
        let res = hdk::remove_link(
            //*hdk::AGENT_ADDRESS,
            &my_handle_address,
            &pending_address,
            link_kind::MailInbox,
            "",
        );
        if let Err(err) = res {
            hdk::debug("Remove ``mail_inbox`` link failed:").ok();
            hdk::debug(err).ok();
            continue;
        }
        //  4. Delete PendingMail entry
        let res = hdk::remove_entry(pending_address);
        if let Err(err) = res {
            hdk::debug("Delete PendingMail failed:").ok();
            hdk::debug(err).ok();
            continue;
        }
    }
    Ok(new_inmails)
}