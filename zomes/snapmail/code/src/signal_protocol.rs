use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
};
use crate::{
    AgentAddress,
    mail::Mail,
};


#[derive(Clone, Deserialize)]
pub enum SignalProtocol {
    ReceivedMail(ReceivedMail),
    ReceivedAck(ReceivedAck),
}

#[derive(Clone, Deserialize)]
pub struct ReceivedMail {
    pub from: AgentAddress,
    pub mail: Mail,
}

#[derive(Clone, Deserialize)]
pub struct ReceivedAck {
    pub from: AgentAddress,
    pub for_mail: Address,
}