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

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub enum InMailState {
    // PendingMail available
    Incoming,
    // InMail written, no pendingMail
    Arrived,
    // OutAck written, PendingAck available
    Acknowledged,
    // OutAck written, no PendingAck
    AckReceived,
    //
    Deleted,
}


#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub enum OutMailState {
    // Has a pending link for each receipient
    Pending,
    // Has less pending links than receipients, and no receipt links
    PartiallyArrived_NoAcknowledgement,
    // Has less pending links than receipients, and less receipt links than receipients
    PartiallyArrived_PartiallyAcknowledged,
    // Has no pending links, and a receipt link for each receipient
    Arrived_NoAcknowledgement,
    // Has no pending links, and less receipt links than receipients
    Arrived_PartiallyAcknowledged,
    // Has no pendings links, and a receipt link for each receipient
    Received,
    //
    Deleted,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub enum MailState {
    In(InMailState),
    Out(OutMailState),
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MailItem {
    pub address: Address,
    pub author: AgentAddress,
    pub mail: Mail,
    pub state: MailState,
    pub bcc: Vec<AgentAddress>,
    pub date: i64,
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
