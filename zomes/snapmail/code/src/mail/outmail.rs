use std::time::SystemTime;

use hdk::{
    error::ZomeApiResult,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        entry::Entry,
        agent::AgentId,
        dna::entry_types::Sharing,
    },
    holochain_persistence_api::{
        cas::content::Address,
    },
};

use super::{
    Mail, pending_mail::PendingMail,
};

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing an authored mail. It is private.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct OutMail {
    mail: Mail,
    bcc: Vec<AgentId>,
}

/// Entry definition
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

//-------------------------------------------------------------------------------------------------
// Implementation
//-------------------------------------------------------------------------------------------------

///
impl OutMail {
    pub fn new(mail: Mail, bcc: Vec<AgentId>) -> Self {
        Self {
            mail, bcc,
        }
    }

    pub fn create(
        subject: String,
        payload: String,
        to_first: AgentId,
        to_remaining: Vec<AgentId>,
        cc: Vec<AgentId>,
        bcc: Vec<AgentId>,
    ) -> ZomeApiResult<(Address, Entry)> {
        let date_sent = crate::snapmail_now();
        let mail = Mail { date_sent, subject, payload, to_first, to_remaining, cc, };
        let outmail = OutMail::new(mail, bcc)?;
        let entry = Entry::App("outmail".into(), outmail.into());
        let address = hdk::commit_entry(&entry)?;
        Ok((address, entry))
    }

    pub fn generate_pendings(self) -> Vec<PendingMail> {
        let mut result = Vec::with_capacity(self.mail.cc.size() + self.bcc.size());
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