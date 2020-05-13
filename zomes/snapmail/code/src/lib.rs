#![feature(proc_macro_hygiene)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_json_derive;

mod mail;
mod handle;
mod utils;
mod protocol;
mod signal_protocol;
mod globals;
mod link_kind;
mod entry_kind;

use hdk::prelude::*;

use hdk::error::ZomeApiError;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_json_api::json::JsonString,
    holochain_persistence_api::{
        cas::content::Address
    },
};

use hdk_proc_macros::zome;

pub use signal_protocol::*;
pub use protocol::*;
pub use utils::*;
pub use globals::*;
//pub use link_kind::*;
pub use entry_kind::*;

use mail::entries::*;

pub type AgentAddress = Address;

#[zome]
mod snapmail {

    // -- System -- //

    use crate::DirectMessageProtocol;


    #[init]
    fn init() {
        // TODO: create initial username with AgentId
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentAddress>) {
        Ok(())
    }

    /// Receive point for one of the Protocol messages
    #[receive]
    pub fn receive(from: Address, msg_json: String) -> String {
        mail::receive(from, JsonString::from_json(&msg_json))
    }

    // -- Entry definitions -- //

    #[entry_def]
    fn handle_def() -> ValidatingEntryType {
        handle::handle_def()
    }

    #[entry_def]
     fn outmail_def() -> ValidatingEntryType {
        mail::entries::outmail_def()
    }

    #[entry_def]
    fn inmail_def() -> ValidatingEntryType {
        mail::entries::inmail_def()
    }

    #[entry_def]
    fn pending_mail_def() -> ValidatingEntryType {
        mail::entries::pending_mail_def()
    }

    #[entry_def]
    fn pending_ack_def() -> ValidatingEntryType {
        mail::entries::pending_ack_def()
    }

    #[entry_def]
    fn outack_def() -> ValidatingEntryType {
        mail::entries::outack_def()
    }

    #[entry_def]
    fn inack_def() -> ValidatingEntryType {
        mail::entries::inack_def()
    }

    // -- Zome Functions -- //

    /// Set handle for this agent
    /// Return address to new or updated Handle Entry
    #[zome_fn("hc_public")]
    fn set_handle(name: String) -> ZomeApiResult<Address> { handle::set_handle(name) }

    /// Get this agent's latest handle
    #[zome_fn("hc_public")]
    fn get_my_handle() -> ZomeApiResult<String> { handle::get_my_handle() }

    /// Get some agent's latest handle
    #[zome_fn("hc_public")]
    fn get_handle(agentId: String) -> ZomeApiResult<String> {
        handle::get_handle(agentId.into())
    }

    #[zome_fn("hc_public")]
    fn get_all_handles() -> ZomeApiResult<Vec<(String, AgentAddress, Address)>> {
        handle::get_all_handles()
    }

    #[zome_fn("hc_public")]
    fn find_agent(handle: String) -> ZomeApiResult<Vec<AgentAddress>> {
        handle::find_agent(handle)
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
    ) -> ZomeApiResult<mail::SendTotalResult> {
        if to.len() + cc.len() + bcc.len() < 1 {
            return Err(ZomeApiError::Internal("Mail lacks receipients".into()))
        }
        mail::send_mail(subject, payload, to, cc, bcc)
    }

    /// Get an InMail or OutMail at given address.
    #[zome_fn("hc_public")]
    fn get_mail(address: Address) -> Option<Result<InMail, OutMail>> {
        mail::get_mail(address)
    }

    /// Get all InMails and OutMails
    #[zome_fn("hc_public")]
    fn get_all_mails() -> ZomeApiResult<Vec<MailItem>> { mail::get_all_mails() }

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

    /// Create & share an Acknowledgmeent for a mail we received.
    /// Return Address of OutAck.
    #[zome_fn("hc_public")]
    fn acknowledge_mail(inmail_address: Address) -> ZomeApiResult<Address> {
        mail::acknowledge_mail(inmail_address)
    }

    /// Check if agent received a receipt from all receipients of one of this agent's OutMail.
    /// If false, returns list of agents who's receipt is missing.
    #[zome_fn("hc_public")]
    fn has_mail_been_received(outmail_address: Address) -> ZomeApiResult<Result<(), Vec<AgentAddress>>> {
        mail::has_mail_been_received(outmail_address)
    }

    /// Check if an InMail's source has received an Acknowledgement from this agent.
    #[zome_fn("hc_public")]
    fn has_ack_been_received(inmail_address: Address) -> ZomeApiResult<bool> {
        mail::has_ack_been_received(inmail_address)
    }
}
