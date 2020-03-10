use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
        agent::AgentId,
    },
};


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
                ),
            ],
    )
}

impl Handle {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}

pub fn get_handle() -> Option<Handle> {
    handle::get_handle()
}

pub fn set_handle(name: String) -> ZomeApiResult<Address> {
    let new_handle = Handle::new(name);
    let maybe_current_handle = get_handle();
    if let Some(current_handle) = maybe_current_handle {
        hdk::update_entry(profile_entry, &address)?;
    }
    let app_entry = Entry::App("handle".into(), goal.clone().into());
    let entry_address = hdk::commit_entry(&app_entry)?;
    return Ok(entry_address);
}

