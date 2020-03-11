
#[derive(Clone, Deserialize)]
pub enum DirectMessageProtocol {
    Mail(MailMessage),
    Ack(AckMessage),
}

#[derive(Clone, Deserialize)]
pub struct MailMessage {
    pub outmail_address: Address,
    pub mail: Mail,
}

#[derive(Clone, Deserialize)]
pub struct AckMessage {
    outmail_address: Address,
    ack_address: Address,
}