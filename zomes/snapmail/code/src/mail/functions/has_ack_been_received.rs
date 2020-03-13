use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::crud_status::CrudStatus,
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
};
use crate::{
    AgentAddress,
    mail::entries::{
        OutMail, InMail,
    },
};

/// Zome function
/// Ack is considered received if there is no pendingAck link or PendingAck has delete status
pub fn has_ack_been_received(inmail_address: &Address) -> ZomeApiResult<bool> {
    // 0. Get InMail
    let inmail = hdk::utils::get_as_type::<inMail>(inmail_address)?;
    // 1. Get OutAck
    let links_result = hdk::get_links(inmail_address,"acknowledgement".into(), LinkMatch::Any)?;
    if links_result.links().size() < 1 {
        return Err(ZomeApiError::Internal("No acknowledgment has been sent for this mail".to_string()));
    }
    let outack_address = links_result.addresses()[0].clone();
    let outack = hdk::utils::get_as_type::<OutMail>(outack_address)?;
    // 2. Get OutAck pending link
    let links_result = hdk::get_links(&outack_address,"pending".into(), LinkMatch::Any)?;
    // 3. If no link than return OK
    if links_result.links().size() < 1 {
        return Ok(true);
    }
    // 4. Otherwise get PendingAck crud status
    let pending_address = links_result.addresses()[0].clone();
    let maybe_pending_history = hdk::get_entry_history(&pending_address)?;
    if maybe_pending_history.is_none() {
        return Err(ZomeApiError::Internal("No history found for PendingAck".to_string()));
    }
    // 5. Return Ok if status == deleted
    let history = maybe_pending_history.unwrap();
    for item in history.items {
        if let Some(meta) = item.meta {
            if meta.crud_status == CrudStatus::Deleted {
                return Ok(true);
            }
        }
    }
    Ok(false)
}