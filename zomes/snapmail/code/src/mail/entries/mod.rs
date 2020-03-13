pub mod inmail;
pub mod outmail;
pub mod pending_mail;
pub mod ackreceipt_encrypted;
pub mod ackreceipt_private;

pub use self::{inmail::*, pending_mail::*, outmail::*, send::*, ackreceipt_encrypted::*};

use crate::AgentAddress;

pub enum OutMailState {
    CREATED,    // OutMail written
    SENT,       // PendingMail created and/or Some AckReceipts have been received
    RECEIVED,   // All AckReceipts havec been received, no more PendingMail
}

pub enum InMailState {
    INCOMING, // PendingMail for this agent
    ARRIVED, // InMail written
    ACKNOWLEDGED, // AckReceipt written
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
    date_sent: u64,
    subject: String,
    payload: String,
    to: Vec<AgentAddress>,
    cc: Vec<AgentAddress>,
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