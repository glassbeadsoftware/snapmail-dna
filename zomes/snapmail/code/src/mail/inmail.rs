use crate::AgentAddress;
use super::{
    Mail, PendingMail,
};

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
    },
};
use hdk::error::ZomeApiError;

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing a received mail. It is private.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct InMail {
    mail: Mail,
    from: AgentAddress,
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
                // FIXME
                Ok(())
            },
            links: [
                to!(
                    "ackreceipt_encrypted",
                    link_type: "acknowledgment_encrypted",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        // FIXME
                        Ok(())
                    }
                ),
                to!(
                    "ackreceipt_private",
                    link_type: "acknowledgment_private",
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

//-------------------------------------------------------------------------------------------------
// Implementation
//-------------------------------------------------------------------------------------------------

impl InMail {
    pub fn new(mail: Mail, from: AgentAddress, date_received: u64) -> Self {
        Self {
            mail,
            from,
            date_received,
        }
    }

    pub fn from_pending(pending: PendingMail, from: AgentAddress) -> Result<Self> {
        let maybe_mail = pending.decrypt(from);
        if maybe_mail.is_err() {
            return ZomeApiError();
        }
        let received_date = crate::snapmail_now();
        Self::new(mail, from.clone(), received_date)
    }
}