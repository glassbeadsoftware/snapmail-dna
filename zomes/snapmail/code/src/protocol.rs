use hdk::prelude::*;
use hdk::holochain_persistence_api::cas::content::Address;

use crate::mail::entries::Mail;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum DirectMessageProtocol {
    Mail(MailMessage),
    Ack(AckMessage),
    Failure(String),
    Success(String),
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MailMessage {
    pub outmail_address: Address,
    pub mail: Mail,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct AckMessage {
    pub outmail_address: Address,
}
