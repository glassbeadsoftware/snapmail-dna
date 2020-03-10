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
    },
};

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

pub fn check_mail_inbox() -> ZomeApiResult<Vec<Address>> {
    // FIXME
    // Lookup `mail_inbox` links on my agentId
    // For each link
    //  - Get entry on the DHT
    //  - Convert and Commit as InMail
    //  - Commit & Share AckReceipt
    //  - Delete PendingMail entry
    //  - Remove link from my agentId
}

pub fn check_ack_inbox() -> ZomeApiResult<Vec<Address>> {
    // FIXME
    // Lookup `ack_inbox` links on my agentId
    // For each link
    //  - Get entry on the DHT
    //  - Add Acknowledgement link to my OutMail
    //  - Delete AckReceipt entry
    //  - Delete link from my agentId
}