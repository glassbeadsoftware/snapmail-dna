use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
};

use crate::mail::Mail;

#[derive(Clone, Deserialize)]
pub enum DirectMessageProtocol {
    Mail(MailMessage),
    Ack(AckMessage),
    Failure(String),
    Success(String),
}

#[derive(Clone, Deserialize)]
pub struct MailMessage {
    pub outmail_address: Address,
    pub mail: Mail,
}

#[derive(Clone, Deserialize)]
pub struct AckMessage {
    pub outmail_address: Address,
}
