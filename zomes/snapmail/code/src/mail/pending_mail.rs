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

use super::Mail;

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing a mail on the DHT waiting to be received
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PendingMail {
    encrypted_mail: String,
}

pub fn pendingmail_def() -> ValidatingEntryType {
    entry!(
        name: "pendingmail",
        description: "Entry for a mail held in the DHT waiting to be received by its receipient",
        sharing: Sharing::Encrypted,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<OutMail>| {
            Ok(())
        }
    )
}

//-------------------------------------------------------------------------------------------------
// Implementation
//-------------------------------------------------------------------------------------------------

impl PendingMail {
    pub fn new(encrypted: string) -> Self {
        Self {
            encrypted_mail: encrypted,
        }
    }

    /// Create PendingMail from Mail and destination AgentId
    /// This will encrypt the Mail with the destination's key
    pub fn create(mail: Mail, _to: AgentId) -> Self {
        // Serialize
        let serialized = serde_json::to_string(mail).unwrap();

        // Encrypt
        let encrypted = serialized;
        // FIXME should be:
        // const encrypted = hdk::encrypt(mail, to);

        // Create
        PendingMail::new(encrypted)
    }

    pub fn decrypt(self, _from: AgentId) -> Result<Mail, ()> {
        // decrypt
        let maybe_decrypted = Ok(self.encrypted_mail);
        // FIXME should be:
        // const maybe_decrypted = hdk::decrypt(self.encrypted_mail, from);
        // if maybe_decrypted.is_err() {
        //     return Err();
        // }
        // deserialize
        let maybe_mail: Result<Mail> = serde_json::from_str(&decrypted.unwrap());
        maybe_mail
    }
}