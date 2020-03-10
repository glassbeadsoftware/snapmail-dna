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
    pub mail: Mail,
    pub bcc: Vec<AgentId>,
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
            // FIXME: Check no duplicate recepient?
            Ok(())
        },
        links: [
            to!(
                "ackreceipt_encrypted",
                link_type: "receipt_encrypted",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    // FIXME: Check AckReceipt author is one of the OutMail's recepient
                    // FIXME: Check if AckReceipt for this author already received?
                    Ok(())
                }
            ),
            to!(
                "ackreceipt_private",
                link_type: "receipt_private",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    // FIXME: Check AckReceipt author is one of the OutMail's recepient
                    // FIXME: Check if AckReceipt for this author already received?
                    Ok(())
                }
            ),
            to!(
                "pendingmail",
                link_type: "pending",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    // FIXME: Check that outmail_address within PendingMail corresponds
                    // FIXME: Check PendingMail is authored by same agant
                    Ok(())
                }
            ),
        ],
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
    ) -> Self {
        // TODO: remove duplicate receipients
        let date_sent = crate::snapmail_now();
        let mail = Mail { date_sent, subject, payload, to_first, to_remaining, cc };
        OutMail::new(mail, bcc)
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