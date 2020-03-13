use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    entry_definition::ValidatingEntryType,
};

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