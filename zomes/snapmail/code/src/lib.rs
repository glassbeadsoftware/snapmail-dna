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

use mail;
use hdk_proc_macros::zome;

// see https://developer.holochain.org/api/0.0.42-alpha5/hdk/ for info on using the hdk library


#[zome]
mod snapmail {
    use std::time::SystemTime;


    #[init]
    fn init() {
        // TODO: create username?
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    // -- outmail -- //

    #[entry_def]
     fn outmail_def() -> ValidatingEntryType {
        mail::outmail_def()
    }

    #[zome_fn("hc_public")]
    fn create_outmail(subject: String, payload: String, to: Vec<AgentId>, cc: Vec<AgentId>, bcc: Vec<AgentId>) -> ZomeApiResult<Address> {
        let date_sent = SystemTime::now();
        let mail = Mail { date_sent, subject, payload, to, cc, };
        let outmail = OutMail::new(mail, bcc)?;
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
        mail::inmail_def()
    }
    
    #[zome_fn("hc_public")]
    fn get_inmail(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

}
