use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
        agent::AgentId,
        dna::entry_types::Sharing,
    }
};

use std::collections::HashMap;
use super::{Mail, OutMail};

pub enum SendResult {
    OK_DIRECT,
    OK_PENDING(Address),
    ERR,
}

/// Struct holding all result data from a send request
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct SendTotalResult {
    outmail: Address,
    to_pendings: HashMap<AgentId, Address>,
    cc_pendings: HashMap<AgentId, Address>,
    bcc_pendings: HashMap<AgentId, Address>,
}

impl SendTotalResult {
    pub fn new(outmail_address: Address) -> Self {
        Self {
            outmail: outmail_address,
            to_pendings: HashMap::new(),
            cc_pendings: HashMap::new(),
            bcc_pendings: HashMap::new(),
        }
    }

    pub fn add_pending(mut self, kind: super::ReceipientKind, agentId: AgentId, address: Address) {
        match kind {
            TO => self.to_pendings.insert(agentId, address),
            CC => self.cc_pendings.insert(agentId, address),
            BCC => self.bcc_pendings.insert(agentId, address),
        };
    }
}

///
fn send_mail_to(mail: &Mail, to_first: AgentId) -> ZomeApiResult<SendResult> {
    let payload = serde_json::to_string(mail).unwrap();
    let result = hdk::send(
        Address::from(to_first.clone()),
        payload,
        crate::DIRECT_SEND_TIMEOUT_MS.into(),
    );
    if let Ok(response) = result {
        return Ok(SendResult::OK_DIRECT);
    };
    Err()
}

/// Zone Function
/// Send Mail: Creates OutMail, tries to send directly to each receipient
/// if not online creates a PendingMail
pub fn send_mail(
    subject: String,
    payload: String,
    to_first: AgentId,
    to: Vec<AgentId>,
    cc: Vec<AgentId>,
    bcc: Vec<AgentId>,
) -> ZomeApiResult<SendTotalResult> {
    let (outmail_address, outmail_entry) = OutMail::create(subject, payload, to_first, to, cc, bcc)?;
    let mut result = SendTotalResult::new(outmail_address);

    // to first
    send_outmail_to(outmail_entry, to_first);

        // to remaining
    for


    // cc

    // bcc


    Ok(outmail_address, pendingMail_address)
}