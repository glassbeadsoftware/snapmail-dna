use hdk::prelude::*;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
};

/// Entry representing an AcknowldegmentReceipt on the DHT waiting to be received
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PendingAck {
    pub outmail_address: Address,
}

pub fn pending_ack_def() -> ValidatingEntryType {
    entry!(
        name: "pending_ack",
        description: "Entry for an Acknowledgement Receipt of a Mail to be stored on the DHT",
        sharing: Sharing::Encrypted,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<PendingAck>| {
            // FIXME
            Ok(())
        },
        links: [
            from!(
                "%agent_id",
                link_type: "ack_inbox",
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

impl PendingAck {
    pub fn new(outmail_address: Address) -> Self {
        Self {
            outmail_address,
        }
    }
}
