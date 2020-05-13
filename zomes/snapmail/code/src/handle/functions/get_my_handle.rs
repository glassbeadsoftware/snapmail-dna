use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
        link::LinkMatch,
    },
};

use crate::{
    link_kind,
    handle::utils::get_handle_string,
};


/// Zome Function
/// get this agent's latest handle
pub fn get_my_handle() -> ZomeApiResult<String> {
    let maybe_current_handle_entry = get_my_handle_entry();
    return get_handle_string(maybe_current_handle_entry);
}

/// Return (handle entry address, handle entry) pair
pub fn get_my_handle_entry() -> Option<(Address, Entry)> {
    let link_results = hdk::get_links(
        &*hdk::AGENT_ADDRESS,
        LinkMatch::Exactly(link_kind::Handle),
        LinkMatch::Any,
    ).expect("No reason for this to fail");
    let links_result = link_results.links();
    assert!(links_result.len() <= 1);
    if links_result.len() == 0 {
        hdk::debug("No handle found for this agent:").ok();
        return None;
    }
    let entry_address = &links_result[0].address;
    let entry = hdk::get_entry(entry_address)
        .expect("No reason for get_entry to crash")
        .expect("Should have it");
    return Some((entry_address.clone(), entry));
}