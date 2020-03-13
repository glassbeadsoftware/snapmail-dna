

///
pub fn receive_direct_ack(from: AgentAddress, ack: AckMessage) -> String {
    // FIXME
}

///
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
    let signal_json = serde_json::to_string(signal).expect("Should stringify");
    hdk::emit_signal("received_mail", JsonString::from_json(&signal_json));

    // Done
    return DirectMessageProtocol::Success(String::new());
}
