use hdk::prelude::*;

use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::entry::Entry,
    holochain_json_api::json::JsonString,
    holochain_persistence_api::{
        cas::content::Address
    },
};
use crate::{
    mail::{
        self,
        entries::{ack::AckReceiptEncrypted, InMail, InAck, }
    },
    AgentAddress, DirectMessageProtocol, MailMessage, AckMessage,
    ReceivedMail, ReceivedAck,
};

/// Handle a MailMessage.
/// Emits `received_mail` signal.
/// Returns Success or Failure.
pub fn receive_direct_mail(from: AgentAddress, mail_msg: MailMessage) -> DirectMessageProtocol {
    // Create InMail
    let inmail = InMail::from_direct(author, mail_msg);
    let inmail_entry = Entry::App("inmail".into(), inmail.into());
    let maybe_inmail_address = hdk::commit_entry(&inmail_entry);
    if let Err(err) = maybe_inmail_address {
        let response_str = "Failed committing InMail";
        hdk::debug(format!("{}: {}", response_str, err));
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    // Emit signal
    let signal = ReceivedMail {
        from: from.clone(),
        mail: mail_msg.mail.clone(),
    };
    let signal_json = serde_json::to_string(&signal).expect("Should stringify");
    hdk::emit_signal("received_mail", JsonString::from_json(&signal_json));
    // Return Success response
    return DirectMessageProtocol::Success(String::new());
}

/// Handle a AckMessage.
/// Emits `received_ack` signal.
/// Returns Success or Failure.
pub fn receive_direct_ack(from: AgentAddress, ack_msg: AckMessage) -> DirectMessageProtocol {
    // Create InAck
    let res = mail::create_and_commit_inack(&ack_msg.outmail_address, &from);
    if let Err(err) = res {
        let response_str = "Failed committing InAck";
        hdk::debug(format!("{}: {}", response_str, err));
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    // Emit Signal
    let signal = ReceivedAck {
        from: from.clone(),
        for_mail: ack_msg.outmail_address.clone(),
    };
    let signal_json = serde_json::to_string(&signal).expect("Should stringify");
    hdk::emit_signal("received_ack", JsonString::from_json(&signal_json));
    // Return Success response
    return DirectMessageProtocol::Success(String::new());
}
