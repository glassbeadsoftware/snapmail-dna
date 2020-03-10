#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

pub mod inmail;
pub mod outmail;
pub mod pending_mail;
pub mod send;
pub mod ack;

pub use self::{inmail::*, pending_mail::*, outmail::*, send::*};

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

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub enum ReceipientKind {
    TO,
    CC,
    BCC,
}

/// Core content of all *Mail Entries
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Mail {
    date_sent: u64,
    subject: String,
    payload: String,
    to_first: AgentId,
    to_remaining: Vec<AgentId>,
    cc: Vec<AgentId>,
}

impl Mail {
    pub fn new(
        date_sent: u64,
        subject: String,
        payload: String,
        to_first: AgentId,
        to_remaining: Vec<AgentId>,
        cc: Vec<AgentId>,
    ) -> Self {
        Self {
            date_sent,
            subject,
            payload,
            to_first,
            to_remaining,
            cc
        }
    }
}