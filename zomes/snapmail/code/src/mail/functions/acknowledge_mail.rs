use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    entry_definition::ValidatingEntryType,
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
    AgentAddress,
    mail::entries::{
        OutMail, InMail, AckReceiptEncrypted,
    },
};

/// Zome function
/// Return address of newly created AckReceipt
pub fn acknowledge_mail(inmail_address: &Address) -> ZomeApiResult<Address> {
    //  1. Make sure its an InMail
    let inmail = hdk::utils::get_as_type::<InMail>(inmail_address.clone())?;
    //  2. Make sure it has not already been acknowledged
    let res_count = hdk::get_links_count(inmail_address, "receipt_private".into(), LinkMatch::Any)?;
    if res.count > 0 {
        return Err(ZomeApiError::Internal("Mail has already been acknowledged (private)".to_string()));
    }
    let res_count = hdk::get_links_count(inmail_address, "receipt_encrypted".into(), LinkMatch::Any)?;
    if res.count > 0 {
        return Err(ZomeApiError::Internal("Mail has already been acknowledged (encrypted)".to_string()));
    }
    // 3. Try Direct Acknowledgment?
    // FIXME
    // 4. Acknowledge via DHT
    return acknowledge_mail_encrypted(inmail_address, &inmail.from);
}

/// Return address of newly created AckReceiptPrivate
fn acknowledge_mail_private(inmail_address: &Address) -> ZomeApiResult<Address> {
    // FIXME
}

/// Create & Commit AckReceiptEncrypted
/// Return address of newly created AckReceiptEncrypted
fn acknowledge_mail_encrypted(inmail_address: &Address, from: &AgentAddress) -> ZomeApiResult<Address> {
    let ack = AckReceiptEncrypted::new(outmail_address.clone());
    let ack_entry = Entry::App("ackreceipt_encrypted".into(), ack.into());
    let ack_address = hdk::commit_entry(&ack_entry)?;
    let _ = hdk::link_entries(&inmail_address, &ack_address, "acknowledgment_encrypted", "")?;
    let _ = hdk::link_entries(&from, &ack_address, "ack_inbox", &HDK::AGENT_ADDRESS)?;
    Ok(ack_address)
}
