mod functions;
mod utils;
mod validation;

pub use functions::*;

use hdk::prelude::*;

use hdk::{
    entry_definition::ValidatingEntryType,
};

use crate::{
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
        validation: | validation_data: hdk::EntryValidationData<Handle>| {
            validation::validate_handle(validation_data)
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
