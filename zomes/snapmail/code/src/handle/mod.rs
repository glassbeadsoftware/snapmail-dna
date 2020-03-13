use hdk::prelude::*;

//use hdk::{
//    error::{ZomeApiError, ZomeApiResult},
//    entry_definition::ValidatingEntryType,
//    holochain_persistence_api::{
//        cas::content::Address
//    },
//    holochain_core_types::{
//        entry::Entry,
//        link::LinkMatch,
//    },
//};

use crate::utils::into_typed;

/// Entry representing the username of an Agent
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Handle {
    pub name: String,
}

pub fn handle_def() -> ValidatingEntryType {
    entry!(
        name: "handle",
        description: "Entry for an Agent's public username",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Handle>| {
            // FIXME
            Ok(())
        },
            links: [
                from!(
                    "%agent_id",
                    link_type: "handle",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        // FIXME: Can only set handle for self
                        Ok(())
                    }
                ),
                from!(
                    "%dna",
                    link_type: "member",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        // FIXME
                        Ok(())
                    }
                )
            ]
    )
}

impl Handle {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}

/// Zome Function
/// get latest handle for this agent
pub fn get_handle() -> Option<(Address, Entry)> {
    let link_results = hdk::get_links(
        &*hdk::AGENT_ADDRESS,
        LinkMatch::Exactly("handle"),
        LinkMatch::Any,
    ).expect("No reason for this to fail");
    let links_result = link_results.links();
    assert!(links_result.len() <= 1);
    if links_result.len() == 0 {
        return None;
    }
    let entry_address = &links_result[0].address;
    let entry = hdk::get_entry(entry_address)
        .expect("No reason to crash here")
        .expect("Should have it");
    return Some((entry_address.clone(), entry));
}

/// Zome Function
/// Set handle for this agent
pub fn set_handle(name: String) -> ZomeApiResult<Address> {
    let new_handle = Handle::new(name);
    let app_entry = Entry::App("handle".into(), new_handle.into());
    let maybe_current_handle_entry = get_handle();
    if let Some((entry_address, current_handle_entry)) = maybe_current_handle {
        // If handle already set to this value, just return current entry address
        let current_handle = into_typed::<Handle>(current_handle_entry)
            .expect("Should be a Handle entry");
        if current_handle.name == name {
            return Ok(entry_address);
        }
        // Really new name so just update entry
        hdk::update_entry(app_entry, &entry_address)?;
    }
    // First Handle ever, commit entry
    let entry_address = hdk::commit_entry(&app_entry)?;
    let _ = hdk::link_entries(&HDK::AGENT_ADDRESS, &ack_address, "handle", "")?;
    return Ok(entry_address);
}

