use hdk::prelude::*;

use hdk::{
    error::ZomeApiResult,
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
        link::LinkMatch,
    },
    holochain_core_types::time::Timeout,
};

use crate::{
    AgentAddress,
    utils::into_typed,
    entry_kind, link_kind,
};

/// Entry representing the username of an Agent
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Handle {
    pub name: String,
}

pub fn handle_def() -> ValidatingEntryType {
    entry!(
        name: entry_kind::Handle,
        description: "Entry for an Agent's public username",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Handle>| {
            // FIXME
            // FIXME: min & max character count
            Ok(())
        },
            links: [
                from!(
                    EntryType::AgentId,
                    link_type: link_kind::Handle,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        // FIXME: Can only set handle for self
                        Ok(())
                    }
                ),
                from!(
                    EntryType::Dna,
                    link_type: link_kind::Members,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        // FIXME: Only one handle per agent
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

fn get_handle_string(maybe_handle_entry: Option<(Address, Entry)>) -> ZomeApiResult<String> {
    if let Some((_, current_handle_entry)) = maybe_handle_entry {
        let current_handle = into_typed::<Handle>(current_handle_entry)
            .expect("Should be a Handle entry");
        return Ok(current_handle.name);
    }
    return Ok("<noname>".to_string());
}

/// Zome Function
/// get an agent's latest handle
pub fn get_handle(agentId: AgentAddress) -> ZomeApiResult<String> {
    let maybe_current_handle_entry = get_handle_entry(&agentId);
    return get_handle_string(maybe_current_handle_entry);
}

/// Return handle entry address and entry
pub fn get_handle_entry(agentId: &AgentAddress) -> Option<(Address, Entry)> {
    let query_result = hdk::query(EntryType::Dna.into(), 0, 0);
    let dna_address = query_result.ok().unwrap()[0].clone();
    hdk::debug(format!("dna_address33: {:?}", dna_address)).ok();
    let entry_opts = GetEntryOptions::new(StatusRequestKind::default(), false, true, Timeout::default());
    let entry_results = hdk::get_links_result(
        //&*hdk::DNA_ADDRESS,
        &dna_address,
        LinkMatch::Exactly(link_kind::Members),
        LinkMatch::Any,
        GetLinksOptions::default(),
        entry_opts,
    ).expect("No reason for this to fail");
    hdk::debug(format!("entry_results33: {:?}", entry_results)).ok();

    // Find handle entry whose author is agentId
    for maybe_entry_result in entry_results {
        if let Ok(entry_result) = maybe_entry_result {
            let item = match entry_result.result {
                GetEntryResultType::Single(result_item) => result_item,
                GetEntryResultType::All(history) => history.items[0].clone(),
            };
            let header = item.headers[0].clone();
            let from = header.provenances()[0].clone();
            if from.source() == agentId.clone() {
                return Some((header.entry_address().clone(), item.entry.unwrap()))
            }
        }
    }
    return None;
}

// pub fn get_handle_entry(agentId: &AgentAddress) -> Option<(Address, Entry)> {
//     get_handle_entry_by_agent(agentId)
// }

/// Return (handle entry address, handle entry) pair
pub fn _get_handle_entry_by_agent(agentId: &AgentAddress) -> Option<(Address, Entry)> {
    let link_results = hdk::get_links(
        agentId,
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

/// Zome Function
/// Set handle for this agent
pub fn set_handle(name: String) -> ZomeApiResult<Address> {
    let new_handle = Handle::new(name.clone());
    let app_entry = Entry::App(entry_kind::Handle.into(), new_handle.into());
    let maybe_current_handle_entry = get_handle_entry(&*hdk::AGENT_ADDRESS);
    if let Some((entry_address, current_handle_entry)) = maybe_current_handle_entry {
        // If handle already set to this value, just return current entry address
        let current_handle = into_typed::<Handle>(current_handle_entry)
            .expect("Should be a Handle entry");
        if current_handle.name == name {
            return Ok(entry_address);
        }
        // Really new name so just update entry
        return hdk::update_entry(app_entry.clone(), &entry_address);
    }
    // First Handle ever, commit entry
    hdk::debug("First Handle for this agent!!!").ok();
    let entry_address = hdk::commit_entry(&app_entry)?;
    let _ = hdk::link_entries(&*hdk::AGENT_ADDRESS, &entry_address, link_kind::Handle, "")?;

    // TODO: hdk::DNA_ADDRESS doesnt work for linking, get the dna entry address
    //hdk::debug(format!("DNA_ADDRESS42: {:?}", &*hdk::DNA_ADDRESS)).ok();
    // let dna_entry = hdk::get_entry(&*hdk::DNA_ADDRESS)?;
    // hdk::debug(format!("dna_entry1: {:?}", dna_entry)).ok();
    let query_result = hdk::query(EntryType::Dna.into(), 0, 0);
    //hdk::debug(format!("query_result42: {:?}", query_result)).ok();
    let dna_address = query_result.ok().unwrap()[0].clone();
    hdk::debug(format!("dna_address31: {:?}", dna_address)).ok();
    let _ = hdk::link_entries(/*&*hdk::DNA_ADDRESS*/ &dna_address, &entry_address, link_kind::Members, "")?;
    return Ok(entry_address);
}

/// Get all known users
/// Return (AgentId -> Handle entry address) Map
pub fn get_all_handles() -> ZomeApiResult<Vec<(String, AgentAddress, Address)>> {
    let query_result = hdk::query(EntryType::Dna.into(), 0, 0);
    let dna_address = query_result.ok().unwrap()[0].clone();
    let entry_opts = GetEntryOptions::new(StatusRequestKind::default(), true, true, Timeout::default());
    let entry_results = hdk::get_links_result(
        //&*hdk::DNA_ADDRESS,
        &dna_address,
        LinkMatch::Exactly(link_kind::Members),
        LinkMatch::Any,
        GetLinksOptions::default(),
        entry_opts,
    ).expect("No reason for this to fail");
    hdk::debug(format!("entry_results55 size: {:?}", entry_results.len())).ok();

    // Find handle entry whose author is agentId
    let mut handle_list = Vec::new();
    // Find handle entry whose author is agentId
    for maybe_entry_result in entry_results {
        if let Ok(entry_result) = maybe_entry_result {
            let item = match entry_result.result {
                GetEntryResultType::Single(result_item) => result_item,
                GetEntryResultType::All(history) => history.items[0].clone(),
            };
            let entry = item.entry.unwrap();
            let handle_entry = crate::into_typed::<Handle>(entry).expect("Should be Handle");
            let header = item.headers[0].clone();
            let from = header.provenances()[0].clone();
            handle_list.push((handle_entry.name.clone(), from.source(), header.entry_address().clone()));
        }
    }
    hdk::debug(format!("handle_map size: {}", handle_list.len())).ok();
    return Ok(handle_list)
}

