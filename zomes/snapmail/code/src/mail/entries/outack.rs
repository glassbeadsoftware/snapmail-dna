use hdk::prelude::*;

use hdk::{
    entry_definition::ValidatingEntryType,
};

/// Entry for an Acknowledgement Receipt of a Mail authored by this agent
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct OutAck {
}

pub fn outack_def() -> ValidatingEntryType {
    entry!(
        name: "outack",
        description: "Entry for an Acknowledgement Receipt of a Mail authored by this agent",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<OutAck>| {
            Ok(())
        },
        links: [
            to!(
                "pending_ack",
                link_type: "pending",
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

impl OutAck {
    pub fn new() -> Self {
        Self {
        }
    }
}