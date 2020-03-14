mod inmail;
mod outmail;
mod pending_mail;
mod pending_ack;
mod outack;
mod inack;

use hdk::prelude::*;

pub use self::{
    inmail::*, pending_mail::*, outmail::*,
    pending_ack::*, inack::*, outack::*,
};

use crate::AgentAddress;

pub enum OutMailState {
    CREATED,    // OutMail written
    SENT,       // PendingMail created and/or Some receipts have been received
    RECEIVED,   // All receipts have been received, no more PendingMail
}

#[allow(non_camel_case_types)]
pub enum InMailState {
    INCOMING, // PendingMail for this agent
    ARRIVED, // InMail written
    ACKNOWLEDGED, // OutAck written
    ACK_RECEIVED, // OutAck has been received, no PendingAck
}


#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub enum ReceipientKind {
    TO,
    CC,
    BCC,
}

/// Core content of all *Mail Entries
/// Mail can have Zero public receipient (but must have at least one public or private receipient)
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Mail {
    pub date_sent: u64,
    pub subject: String,
    pub payload: String,
    pub to: Vec<AgentAddress>,
    pub cc: Vec<AgentAddress>,
}

impl Mail {
    pub fn new(
        date_sent: u64,
        subject: String,
        payload: String,
        to: Vec<AgentAddress>,
        cc: Vec<AgentAddress>,
    ) -> Self {
        Self {
            date_sent,
            subject,
            payload,
            to,
            cc
        }
    }
}