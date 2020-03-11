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
use std::time::SystemTime;

mod mail;
mod handle;
pub mod utils;
pub mod globals;
pub use utils::*;
pub use globals::*;

pub type AgentAddress = Address;

// see https://developer.holochain.org/api/0.0.42-alpha5/hdk/ for info on using the hdk library

#[zome]
mod snapmail {

    // -- System -- //

    use hdk::error::ZomeApiError;


    #[init]
    fn init() {
        // TODO: create username?
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentAddress>) {
        Ok(())
    }

    #[receive]
    pub fn receive(from: Address, msg_json: JsonString) -> String {
        hdk::debug(format!("Received from: {:?}", from)).ok();
    }

    // -- Entry definitions -- //

    #[entry_def]
    fn handle_def() -> ValidatingEntryType {
        handle::handle_def()
    }

    #[entry_def]
     fn outmail_def() -> ValidatingEntryType {
        mail::outmail_def()
    }

    #[entry_def]
    fn inmail_def() -> ValidatingEntryType {
        mail::inmail_def()
    }

    #[entry_def]
    fn pendingmail_def() -> ValidatingEntryType {
        mail::pendingmail_def()
    }

    #[entry_def]
    fn ackreceipt_encrypted_def() -> ValidatingEntryType {
        mail::ackreceipt_encrypted_def()
    }

    #[entry_def]
    fn ackreceipt_private_def() -> ValidatingEntryType {
        mail::ackreceipt_private_def()
    }

    // -- Zome Functions -- //

    /// Set handle for this agent
    /// Return address to new or updated Handle Entry
    #[zome_fn("hc_public")]
    fn set_handle(name: String) -> ZomeApiResult<Address> { handle::set_handle(name) }

    #[zome_fn("hc_public")]
    fn get_handle() -> String {
        let maybe_current_handle_entry = handle::get_handle();
        if let Some(current_handle_entry) = maybe_current_handle {
            let current_handle = into_typed::<Handle>(current_handle_entry)
                .expect("Should be a Handle entry");
            return current_handle.name;
        }
        return "<noname>".to_string();
    }

    #[zome_fn("hc_public")]
    fn send_mail(
        subject: String,
        payload: String,
        to: Vec<AgentAddress>,
        cc: Vec<AgentAddress>,
        bcc: Vec<AgentAddress>,
    ) -> ZomeApiResult<Address, Address> {
        if to.size() + cc.size() + bcc.size() < 1 {
            return ZomeApiError::Internal("Mail lacks receipients".into())
        }
        mail::send_mail(subject, payload, to, cc, bcc)
    }

    #[zome_fn("hc_public")]
    fn get_mail(address: Address) -> Option<Result<InMail, OutMail>> {
        mail::get_mail(address)
    }

    #[zome_fn("hc_public")]
    fn check_mail_inbox() -> ZomeApiResult<Vec<Address>> {
        mail::check_mail_inbox()
    }

    #[zome_fn("hc_public")]
    fn check_ack_inbox() -> ZomeApiResult<Vec<Address>> {
        mail::check_ack_inbox()
    }
}
