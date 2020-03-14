use hdk::prelude::*;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
    },
};

use crate::{
    AgentAddress,
    mail::entries::Mail,
};

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing an authored mail. It is private.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct OutMail {
    pub mail: Mail,
    pub bcc: Vec<AgentAddress>,
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
                "inack",
                link_type: "receipt",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    // FIXME: Check if receipt for this author already received?
                    Ok(())
                }
            ),
            to!(
                "pending_mail",
                link_type: "pending",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    // FIXME: Check that outmail_address within PendingMail corresponds
                    // FIXME: Check PendingMail is authored by same agant
                    Ok(())
                }
            )
        ]
    )
}

//-------------------------------------------------------------------------------------------------
// Implementation
//-------------------------------------------------------------------------------------------------

///
impl OutMail {
    pub fn new(mail: Mail, bcc: Vec<AgentAddress>) -> Self {
        Self {
            mail, bcc,
        }
    }

    pub fn create(
        subject: String,
        payload: String,
        to: Vec<AgentAddress>,
        cc: Vec<AgentAddress>,
        bcc: Vec<AgentAddress>,
    ) -> Self {
        assert_ne!(0, to.len() + cc.len() + bcc.len());
        // TODO: remove duplicate receipients
        let date_sent = crate::snapmail_now();
        let mail = Mail { date_sent, subject, payload, to, cc };
        OutMail::new(mail, bcc)
    }
}