use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
    },
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
};
use crate::{
    AgentAddress,
    mail::{
        OutMail, InMail,
    },
};

/// Entry representing an AcknowldegmentReceipt on the DHT waiting to be received
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct AckReceiptEncrypted {
    outmail_address: Address,
}

pub fn ackreceipt_encrypted_def() -> ValidatingEntryType {
    entry!(
        name: "ackreceipt_encrypted",
        description: "Entry for an Acknowledgement Receipt of a Mail to be stored on the DHT",
        sharing: Sharing::Encrypted,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<AckReceiptEncrypted>| {
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
            ),
        ],
    )
}

impl AckReceiptEncrypted {
    pub fn new(outmail_address: Address) -> Self {
        Self {
            outmail_address,
        }
    }
}
