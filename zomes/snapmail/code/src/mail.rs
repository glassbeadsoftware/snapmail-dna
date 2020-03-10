#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    agent::AgentId,
    dna::entry_types::Sharing,
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};
use std::time::SystemTime;

//-------------------------------------------------------------------------------------------------
// Definitions
//-------------------------------------------------------------------------------------------------

/// Core content of *Mail entries
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Mail {
    date_sent: u64,
    subject: String,
    payload: String,
    to: Vec<AgentId>,
    cc: Vec<AgentId>,
}

/// Entry representing an authored mail. It is private.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct OutMail {
    mail: Mail,
    bcc: Vec<AgentId>,
}
pub fn outmail_def() -> ValidatingEntryType {
    entry!(
            name: "outmail",
            description: "Entry for a mail authored by this agent",
            sharing: Sharing::Private,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<OutMail>| {
                Ok(())
            }
        )
}

/// Entry representing a received mail. It is private.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct InMail {
    mail: Mail,
    from: AgentId,
    date_received: u64,
}

pub fn inmail_def() -> ValidatingEntryType {
    entry!(
            name: "inmail",
            description: "Entry for a received mail",
            sharing: Sharing::Private,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<InMail>| {
                Ok(())
            }
        )
}

/// Entry representing a mail on the DHT waiting to be received
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PendingMail {
    encrypted_mail: String,
}

//-------------------------------------------------------------------------------------------------
// Implementations
//-------------------------------------------------------------------------------------------------

impl Mail {
    fn new(
        date_sent: u64,
        subject: String,
        payload: String,
        to: Vec<AgentId>,
        cc: Vec<AgentId>,
    ) -> Self {
        Self {
            date_sent,
            subject,
            payload,
            to,
            cc
        }
    }
}

impl OutMail {
    fn new(mail: Mail, bcc: Vec<AgentId>) -> Self {
        Self {
            mail, bcc,
        }
    }

    fn generate_pendings(self) -> Vec<PendingMail> {
        let result = Vec::new(self.mail.cc.size() + self.bcc.size());
        // cc
        for agent in self.mail.cc {
            let pending = PendingMail::create(self.mail, agent);
            result.push(pending);
        }
        // bcc
        for agent in self.bcc {
            let pending = PendingMail::create(self.mail, agent);
            result.push(pending);
        }
        result
    }
}

impl InMail {
    fn new(mail: Mail, from: AgentId, date_received: u64) -> Self {
        Self {
            mail,
            from,
            date_received,
        }
    }

    fn from_pending(pending: PendingMail, from: AgentId) -> Result<Self> {
        let maybe_mail = pending.decrypt(from);
        if maybe_mail.is_err() {
            return Err();
        }
        let received_date = SystemTime::now();
        Self::new(mail, from, received_date)
    }
}

impl PendingMail {
    fn new(encrypted: string) -> Self {
        Self {
            encrypted_mail: encrypted,
        }
    }

    fn decrypt(self, _from: AgentId) -> Result<Mail> {
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

    /// Create PendingMail from Mail and destination AgentId
    /// This will encrypt the Mail with the destination's key
    fn create(self, mail: Mail, _to: AgentId) -> Self {
        // Serialize
        let serialized = serde_json::to_string(mail).unwrap();

        // Encrypt
        let encrypted = serialized;
        // FIXME should be:
        // const encrypted = hdk::encrypt(mail, to);

        // Ctor
        PendingMail::new(encrypted)
    }
}