#![feature(proc_macro_hygiene)]
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

use hdk_proc_macros::zome;

// see https://developer.holochain.org/api/0.0.42-alpha5/hdk/ for info on using the hdk library

/// Core content of *Mail entries
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Mail {
    subject: String,
    payload: String,
    date_sent: u64,
    to: Vec<AgentId>,
    cc: Vec<AgentId>,
}

/// Entry representing an authored mail. It is private.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct OutMail {
    mail: Mail,
    bcc: Vec<AgentId>,
}

/// Entry representing a received mail. It is private.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct InMail {
    mail: Mail,
    from: AgentId,
    date_received: u64,
}

#[zome]
mod snapmail {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    // -- outmail -- //

    #[entry_def]
     fn outmail_def() -> ValidatingEntryType {
        entry!(
            name: "outmail",
            description: "Entry for an authored mail",
            sharing: Sharing::Private,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<OutMail>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn create_outmail(outmail: OutMail) -> ZomeApiResult<Address> {
        let entry = Entry::App("outmail".into(), outmail.into());
        let address = hdk::commit_entry(&entry)?;
        Ok(address)
    }

    #[zome_fn("hc_public")]
    fn get_outmail(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

    // -- inmail -- //

    #[entry_def]
    fn inmail_def() -> ValidatingEntryType {
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

    #[zome_fn("hc_public")]
    fn create_inmail(inmail: InMail) -> ZomeApiResult<Address> {
        let entry = Entry::App("outmail".into(), inmail.into());
        let address = hdk::commit_entry(&entry)?;
        Ok(address)
    }

    #[zome_fn("hc_public")]
    fn get_inmail(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

}
