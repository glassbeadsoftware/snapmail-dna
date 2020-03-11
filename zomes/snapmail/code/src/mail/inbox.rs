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


fn get_pending_mail(pending_address: &Address) -> ZomeApiResult<(Address, PendingMail)> {
    let get_options = GetEntryOptions {
        status_request: StatusRequestKind::Latest,
        entry: true,
        headers: true,
        timeout: Timeout::default(),
    };
    let maybe_entry_result = hdk::get_entry_result(pending_address, get_options);
    if maybe_entry_result.is_err() {
        hdk::debug("Failed getting pending_address");
        return Err(ZomeApiError::Internal("Failed getting pending_address".into()));
    }
    let entry_result = maybe_entry_result.unwrap();
    let entry_item = match entry_result.result {
        GetEntryResultType::Single(item) => {
            item
        },
        _ => panic!("Asked for latest so should get Single"),
    };
    let pending = crate::into_typed::<PendingMail>(entry_item.entry.unwrap()).expect("Should be PendingMail");
    assert!(entry_item.headers.size() > 0);
    assert!(entry_item.headers[0].provenances()[0] > 0);
    let author = entry_item.headers[0].provenances()[0].source();
    Ok((author, pending))
}

pub fn check_mail_inbox() -> ZomeApiResult<Vec<Address>> {
    // FIXME
    // Lookup `mail_inbox` links on my agentId
    let links_result = hdk::get_links(&HDK::AGENT_ADDRESS, LinkMatch::Exactly("mail_inbox"), LinkMatch::Any)?;
    // For each link
    let mut res = Vec::new();
    for pending_address in &links_result.addresses() {
        //  1. Get entry on the DHT
        let (author, pending) = get_pending_mail(pending_address)?;
        //  2. Convert and Commit as InMail
        let inmail = InMail::from_pending(pending, author);
        //  3. Commit & Share AckReceipt
        //  4. Delete PendingMail entry
        //  5. Remove link from my agentId
    }
    Ok(res)
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