use super::{
    Mail, PendingMail, InMail, OutMail,
};

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
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
    api_serialization::get_entry::{
        GetEntryOptions, StatusRequestKind, GetEntryResultType,
    },
};
use crate::mail::ack::AckReceiptEncrypted;


/// Get InMail our OutMail struct in local source chain at address
pub fn get_mail(address: Address) -> Option<Result<InMail, OutMail>> {
    let maybe_InMail = hdk::utils::get_as_type::<InMail>(address.clone());
    if let Ok(inmail) = maybe_InMail {
        return Some(Ok(inmail));
    }
    let maybe_OutMail = hdk::utils::get_as_type::<OutMail>(address);
    if let Ok(outmail) = maybe_OutMail {
        return Some(Err(outmail));
    }
    None
}


/// Conditions: Must be a single author entry type
fn get_entry_and_author(address: &Address) -> ZomeApiResult<(AgentAddress, Entry)> {
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

fn get_pending_mail(pending_address: &Address) -> ZomeApiResult<(AgentAddress, PendingMail)> {
    let (author, entry) = get_entry_and_author(pending_address)?;
    let pending = crate::into_typed::<PendingMail>(entry.unwrap()).expect("Should be PendingMail");
    Ok((author, pending))
}

fn get_ack_encrypted(ack_address: &Address) -> ZomeApiResult<(AgentAddress, AckReceiptEncrypted)> {
    let (author, entry) = get_entry_and_author(ack_address)?;
    let ack = crate::into_typed::<AckReceiptEncrypted>(entry.unwrap()).expect("Should be AckReceiptEncrypted");
    Ok((author, ack))
}

/// Return list of new InMail addresses
pub fn check_mail_inbox() -> ZomeApiResult<Vec<Address>> {
    // Lookup `mail_inbox` links on my agentId
    let links_result = hdk::get_links(&HDK::AGENT_ADDRESS, LinkMatch::Exactly("mail_inbox"), LinkMatch::Any)?;
    // For each link
    let mut new_inmails = Vec::new();
    for pending_address in &links_result.addresses() {
        //  1. Get entry on the DHT
        let res = get_pending_mail(pending_address);
        if let Err(err) = res {
            continue;
        }
        let (author, pending) = res.unwrap();
        //  2. Convert and Commit as InMail
        let inmail = InMail::from_pending(pending, author);
        let inmail_entry = Entry::App("inmail".into(), inmail.into());
        let maybe_inmail_address = hdk::commit_entry(&inmail_entry);
        if maybe_inmail_address.is_err() {
            hdk::debug("Failed committing inMail");
            continue;
        }
        new_inmails.push(maybe_inmail_address.unwrap());
        //  3. Remove link from my agentId
        let res = hdk::remove_link(
            &AGENT_ADDRESS,
            &pending_address,
            "mail_inbox",
            LinkMatch::Any,
        );
        if let Err(err) = res {
            hdk::debug("Remove ``mail_inbox`` link failed:");
            hdk::debug(err);
            continue;
        }
        //  4. Delete PendingMail entry
        let res = hdk::remove_entry(pending_address);
        if let Err(err) = res {
            hdk::debug("Delete PendingMail failed:");
            hdk::debug(err);
            continue;
        }
    }
    Ok(new_inmails)
}

/// Return list of AckReceiptEncryted addresses
pub fn check_ack_inbox() -> ZomeApiResult<Vec<Address>> {
    // Lookup `ack_inbox` links on my agentId
    let links_result = hdk::get_links(&HDK::AGENT_ADDRESS, LinkMatch::Exactly("ack_inbox"), LinkMatch::Any)?;
    // For each link
    let mut new_acks = Vec::new();
    for ack_address in &links_result.addresses() {
        //  - Get entry on the DHT
        let res = get_ack_encrypted(ack_address);
        if let Err(err) = res {
            continue;
        }
        let (author, ack) = res.unwrap();
        //  - Add Acknowledgement link to my OutMail
        let res = hdk::link_entries(&HDK::AGENT_ADDRESS, &ack_address, "receipt_encrypted", "");
        if let Err(err) = res {
            hdk::debug("Add ``receipt_encrypted`` link failed:");
            hdk::debug(err);
            continue;
        }
        //  - Delete AckReceipt link from my agentId
        let res = hdk::remove_link(
            &AGENT_ADDRESS,
            &ack_address,
            "ack_inbox",
            LinkMatch::Any,
        );
        if let Err(err) = res {
            hdk::debug("Remove ``ack_inbox`` link failed:");
            hdk::debug(err);
            continue;
        }
    }
    Ok(new_acks)
}

/// Return address of newly created AckReceiptEncrypted
pub fn acknowledge_mail(inmail_address: &Address) -> ZomeApiResult<Address> {
    //  1. Get InMail
    let maybe_InMail = hdk::utils::get_as_type::<InMail>(address.clone())?;
    //  2. Make sure it has not already been acknowledged
    let res_count = hdk::get_links_count(inmail_address, "receipt_private".into(), LinkMatch::Any)?;
    if res.count > 0 {
        return Err(ZomeApiError::Internal("Mail has already been acknowledged (private)".to_string()));
    }
    let res_count = hdk::get_links_count(inmail_address, "receipt_encrypted".into(), LinkMatch::Any)?;
    if res.count > 0 {
        return Err(ZomeApiError::Internal("Mail has already been acknowledged (encrypted)".to_string()));
    }
    //  3. Create & Commit AckReceiptEncrypted
    let ack = AckReceiptEncrypted::new(outmail_address.clone());
    let ack_entry = Entry::App("ackreceipt_encrypted".into(), ack.into());
    let ack_address = hdk::commit_entry(&ack_entry)?;
    Ok(ack_address)
}

///
pub fn receive_direct_mail(from: AgentAddress, msg_json: JsonString) -> String {
    // FIXME
    String::new()
}

