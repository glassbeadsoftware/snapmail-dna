use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
        dna::entry_types::Sharing,
    }
};

use std::collections::HashMap;
use super::{Mail, OutMail};
use hdk::error::ZomeApiError;
use holochain_wasm_utils::holochain_persistence_api::hash::HashString;
use crate::mail::{PendingMail, ReceipientKind};

use crate::AgentAddress;
use crate::protocol::{MailMessage, DirectMessageProtocol};


pub enum SendSuccessKind {
    OK_DIRECT,
    OK_PENDING(Address),
}

/// Struct holding all result data from a send request
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct SendTotalResult {
    outmail: Address,
    to_pendings: HashMap<AgentAddress, Address>,
    cc_pendings: HashMap<AgentAddress, Address>,
    bcc_pendings: HashMap<AgentAddress, Address>,
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

    pub fn add_pending(mut self, kind: super::ReceipientKind, agentId: &AgentAddress, address: Address) {
        match kind {
            TO => self.to_pendings.insert(agentId.clone(), address),
            CC => self.cc_pendings.insert(agentId.clone(), address),
            BCC => self.bcc_pendings.insert(agentId.clone(), address),
        };
    }
}

///
fn send_mail_to(outmail_address: &Address, mail: &Mail, destination: &AgentAddress) -> ZomeApiResult<SendSuccessKind> {
    // 1. First try sending directly to other Agent if Online
    //   a. Create DM
    let msg = MailMessage {
        outmail_address: outmail_address.clone(),
        mail: Mail.clone(),
    };
    let payload = serde_json::to_string(DirectMessageProtocol::Mail(msg)).unwrap();
    //   b. Send DM
    let result = hdk::send(
        destination.clone(),
        payload,
        crate::DIRECT_SEND_TIMEOUT_MS.into(),
    );
    //   c. Check Response
    if let Ok(response) = result {
        hdk::debug(format!("Received response: {:?}", response)).ok();
        let maybe_msg: Result<DirectMessageProtocol, _> = msg_json.try_into();
        if let Ok(msg) = maybe_msg {
            if let DirectMessageProtocol::Success(_) = msg {
                return Ok(SendResult::OK_DIRECT);
            }
        }
    };
    // 2. Direct Send failed, so send to DHT instead by creating a PendingMail
    let pending = PendingMail::new(mail.clone(), outmail_address.clone());
    let pending_entry = Entry::App("pending_mail".into(), outmail.into());
    let pending_address = hdk::commit_entry(&pending_entry)?;
    let _ = hdk::link_entries(&outmail_address, &pending_address, "pending", pending_address.into())?;
    let _ = hdk::link_entries(&destination, &pending_address, "mail_inbox", &HDK::AGENT_ADDRESS)?;
    Ok(SendResult::OK_PENDING(pending_address))
}

/// Zone Function
/// Send Mail: Creates OutMail, tries to send directly to each receipient
/// if not online creates a PendingMail
pub fn send_mail(
    subject: String,
    payload: String,
    to: Vec<AgentAddress>,
    cc: Vec<AgentAddress>,
    bcc: Vec<AgentAddress>,
) -> ZomeApiResult<SendTotalResult> {
    let outmail = OutMail::create(subject, payload, to, cc, bcc);
    let outmail_entry = Entry::App("outmail".into(), outmail.into());
    let outmail_address = hdk::commit_entry(&outmail_entry)?;

    let mut total_result = SendTotalResult::new(outmail_address.clone());

    // to
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