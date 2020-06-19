use hdk::{
    holochain_persistence_api::{
        cas::content::Address
    },
};

use crate::{
    handle::Handle,
};

/// Zome function
pub fn get_my_handle_history(initial_handle_address: Address) -> Vec<String> {

    let history_result = hdk::get_entry_history(&initial_handle_address);
    if let Err(_e) = history_result {
        hdk::debug("get_entry_history() failed").ok();
        return Vec::new();
    }
    let maybe_history = history_result.unwrap();
    if maybe_history.is_none() {
        hdk::debug("No history found").ok();
        return Vec::new();
    }
    let history = maybe_history.unwrap();
    hdk::debug(format!("History length: {}", history.items.len())).ok();
    hdk::debug(format!("History crud_links length: {}", history.crud_links.len())).ok();

    let mut handle_list = Vec::new();

    for item in history.items {
        let handle_entry = item.entry.expect("should have entry");
        hdk::debug(format!("History headers length: {}", item.headers.len())).ok();
        hdk::debug(format!("item crud status: {:?}", item.meta.unwrap().crud_status)).ok();
        let handle = crate::into_typed::<Handle>(handle_entry).expect("Should be Handle");
        handle_list.push(handle.name);
    }
    hdk::debug(format!("handle_list = {}", handle_list.len())).ok();
    handle_list
}