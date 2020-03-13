use hdk::prelude::*;

use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    entry_definition::ValidatingEntryType,
};

/// Entry for a received Acknowledgement Receipt
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct InAck {
}

pub fn inack_def() -> ValidatingEntryType {
    entry!(
        name: "inack",
        description: "Entry for a received Acknowledgement Receipt",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<InAck>| {
            Ok(())
        }
    )
}

impl InAck {
    pub fn new() -> Self {
        Self {
        }
    }
}