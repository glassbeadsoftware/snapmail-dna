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

//-------------------------------------------------------------------------------------------------
// AckReceiptEncrypted
//-------------------------------------------------------------------------------------------------

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
            Ok(())
        }
    )
}

impl AckReceiptEncrypted {
    pub fn new(outmail: Address) -> Self {
        Self {
            outmail_address: outmail,
        }
    }
}


//-------------------------------------------------------------------------------------------------
// AckReceiptPrivate
//-------------------------------------------------------------------------------------------------

/// Entry representing an AcknowldegmentReceipt private to to the agent receiving the Mail
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct AckReceiptPrivate {
}

pub fn ackreceipt_private_def() -> ValidatingEntryType {
    entry!(
        name: "ackreceipt_private",
        description: "Entry for an Acknowledgement Receipt of a Mail to stay private on source chain",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<AckReceiptPrivate>| {
            Ok(())
        }
    )
}

impl AckReceiptPrivate {
    pub fn new() -> Self {
        Self {
        }
    }
}