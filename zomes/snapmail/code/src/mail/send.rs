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
use hdk::error::ZomeApiError;
use holochain_wasm_utils::holochain_persistence_api::hash::HashString;
use crate::mail::{PendingMail, ReceipientKind};


pub enum SendSuccessKind {
    OK_DIRECT(Address),
    OK_PENDING(Address),
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

    pub fn add_pending(mut self, kind: super::ReceipientKind, agentId: &AgentId, address: Address) {
        match kind {
            TO => self.to_pendings.insert(agentId.clone(), address),
            CC => self.cc_pendings.insert(agentId.clone(), address),
            BCC => self.bcc_pendings.insert(agentId.clone(), address),
        };
    }
}

///
fn send_mail_to(outmail_address: &Address, mail: &Mail, to_first: &AgentId) -> ZomeApiResult<SendSuccessKind> {
    // First try sending directly to other Agent if Online
    let payload = serde_json::to_string(mail).unwrap();
    let result = hdk::send(
        Address::from(to_first.clone()),
        payload,
        crate::DIRECT_SEND_TIMEOUT_MS.into(),
    );
    if let Ok(response) = result {
        // response should be AckReceiptPrivate address
        let ack_address = HashString::from(response);
        hdk::link_entries(&outmail_address, &ack_address, "receipt_private", "")?;
        return Ok(SendResult::OK_DIRECT(ack_address));
    };
    // Direct Send failed, so send to DHT instead by creating a PendingMail
    let pending = PendingMail::new(mail.clone(), outmail_address.clone());
    let pending_entry = Entry::App("pendingmail".into(), outmail.into());
    let pending_address = hdk::commit_entry(&pending_entry)?;
    Ok(SendResult::OK_PENDING(pending_address))
}

/// Zone Function
/// Send Mail: Creates OutMail, tries to send directly to each receipient
/// if not online creates a PendingMail
pub fn send_mail(
    subject: String,
    payload: String,
    to_first: AgentId,
    to_remaining: Vec<AgentId>,
    cc: Vec<AgentId>,
    bcc: Vec<AgentId>,
) -> ZomeApiResult<SendTotalResult> {
    let outmail = OutMail::create(subject, payload, to_first, to, cc, bcc);
    let outmail_entry = Entry::App("outmail".into(), outmail.into());
    let outmail_address = hdk::commit_entry(&outmail_entry)?;

    let mut total_result = SendTotalResult::new(outmail_address.clone());

    // to first
    let res = send_mail_to(&outmail_address, &outmail.mail, &to_first);
    if let Ok(SendSuccessKind::OK_PENDING(pending_address)) = res {
        total_result.add_pending(ReceipientKind::TO, &to_first, pending_address);
    }

    // to remaining
    for agent in to_remaining {
        let res = send_mail_to(&outmail_address, &outmail.mail, &agent);
        if let Ok(SendSuccessKind::OK_PENDING(pending_address)) = res {
            total_result.add_pending(ReceipientKind::TO, &agent, pending_address);
        }
    }

    // cc
    for agent in cc {
        let res = send_mail_to(&outmail_address, &outmail.mail, &agent);
        if let Ok(SendSuccessKind::OK_PENDING(pending_address)) = res {
            total_result.add_pending(ReceipientKind::CC, &agent, pending_address);
        }
    }

    // bcc
    for agent in bcc {
        let res = send_mail_to(&outmail_address, &outmail.mail, &agent);
        if let Ok(SendSuccessKind::OK_PENDING(pending_address)) = res {
            total_result.add_pending(ReceipientKind::BCC, &agent, pending_address);
        }
    }

    // Done
    Ok(total_result)
}