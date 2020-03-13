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
mod utils;
mod protocol;
mod signal_protocol;
mod globals;

pub use signal_protocol::*;
pub use protocol::*;
pub use utils::*;
pub use globals::*;

pub type AgentAddress = Address;

// see https://developer.holochain.org/api/0.0.42-alpha5/hdk/ for info on using the hdk library

#[zome]
mod snapmail {

    // -- System -- //

    use hdk::error::ZomeApiError;
    use crate::AgentAddress;


    #[init]
    fn init() {
        // TODO: create initial username? (random?)
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentAddress>) {
        Ok(())
    }

    /// Receive point for one of the Protocol messages
    #[receive]
    pub fn receive(from: Address, msg_json: JsonString) -> String {
        hdk::debug(format!("Received from: {:?}", from)).ok();
        let maybe_msg: Result<DirectMessageProtocol, _> = msg_json.try_into();
        if let Err(err) = maybe_msg {
            return format!("error: {}", err);
        }
        let message = match maybe_msg.unwrap() {
            DirectMessageProtocol::MailMessage(mail) => {
                mail::receive_direct_mail(from, mail)
            },
            DirectMessageProtocol::AckMessage(ack) => {
                mail::receive_direct_ack(from, ack)
            }
        };
        let msg_json = serde_json::to_string(message).expect("Should stringify");
        msg_json
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
    fn pending_mail_def() -> ValidatingEntryType {
        mail::pending_mail_def()
    }

    #[entry_def]
    fn pending_ack_def() -> ValidatingEntryType {
        mail::pending_ack_def()
    }

    #[entry_def]
    fn outack_def() -> ValidatingEntryType {
        mail::outack_def()
    }

    #[entry_def]
    fn inack_def() -> ValidatingEntryType {
        mail::inack_def()
    }

    // -- Zome Functions -- //

    /// Set handle for this agent
    /// Return address to new or updated Handle Entry
    #[zome_fn("hc_public")]
    fn set_handle(name: String) -> ZomeApiResult<Address> { handle::set_handle(name) }

    /// Get this agent's latest handle
    #[zome_fn("hc_public")]
    fn get_handle() -> String {
        let maybe_current_handle_entry = handle::get_handle();
        if let Some((_, current_handle_entry)) = maybe_current_handle {
            let current_handle = into_typed::<Handle>(current_handle_entry)
                .expect("Should be a Handle entry");
            return current_handle.name;
        }
        return "<noname>".to_string();
    }

    /// Send mail to all receipients
    /// Returns Map of PendingMail entry per receipient
    /// Conditions: Mail must have at least one receipient
    #[zome_fn("hc_public")]
    fn send_mail(
        subject: String,
        payload: String,
        to: Vec<AgentAddress>,
        cc: Vec<AgentAddress>,
        bcc: Vec<AgentAddress>,
    ) -> ZomeApiResult<SendTotalResult> {
        if to.size() + cc.size() + bcc.size() < 1 {
            return ZomeApiError::Internal("Mail lacks receipients".into())
        }
        mail::send_mail(subject, payload, to, cc, bcc)
    }

    /// Get an InMail or OutMail at given address.
    #[zome_fn("hc_public")]
    fn get_mail(address: Address) -> Option<Result<InMail, OutMail>> {
        mail::get_mail(address)
    }

    /// Return list of all InMails that this agent did not acknowledge.
    #[zome_fn("hc_public")]
    fn get_all_arrived_mail() -> ZomeApiResult<Vec<Address>> {
        mail::get_all_arrived_mail()
    }

    /// Check PendingMails sent to this agent.
    /// Converts each into an InMail.
    /// Return list of created InMail entries.
    #[zome_fn("hc_public")]
    fn check_incoming_mail() -> ZomeApiResult<Vec<Address>> {
        mail::check_incoming_mail()
    }

    /// Check for PendingAcks sent to this agent.
    /// Converts each into an InAck.
    /// Return list of outMail addresses for which we succesfully linked a new InAck
    #[zome_fn("hc_public")]
    fn check_incoming_ack() -> ZomeApiResult<Vec<Address>> {
        mail::check_incoming_ack()
    }

//    /// Create & share an Acknowledgmeent for a mail we received.
//    /// Return Address of OutAck.
//    #[zome_fn("hc_public")]
//    fn acknowledge_mail(inmail_address: &Address) -> ZomeApiResult<Address> {
//        mail::acknowledge_mail(inmail_address)
//    }

//    /// Check if agent received AckReceipts from all receipients of one of this agent's OutMail.
//    /// If false, returns list of agents who's receipt is missing.
//    #[zome_fn("hc_public")]
//    fn has_mail_been_received(outmail_address: &Address) -> ZomeApiResult<Result<(), Vec<AgentAddress>>> {
//        mail::has_mail_been_received(outmail_address)
//    }

//    /// Check if an InMail's source has received an Acknowledgement from this agent.
//    #[zome_fn("hc_public")]
//    fn has_ack_been_received(inmail_address: &Address) -> ZomeApiResult<bool> {
//        mail::has_ack_been_received(inmail_address)
//    }
}
