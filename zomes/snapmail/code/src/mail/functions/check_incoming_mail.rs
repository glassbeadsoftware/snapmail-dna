
/// Return list of new InMail addresses
pub fn check_incoming_mail() -> ZomeApiResult<Vec<Address>> {
    // Lookup `mail_inbox` links on my agentId
    let links_result = hdk::get_links(&HDK::AGENT_ADDRESS, LinkMatch::Exactly("mail_inbox"), LinkMatch::Any)?;
    // For each link
    let mut new_inmails = Vec::new();
    for pending_address in &links_result.addresses() {
        //  1. Get entry on the DHT
        let res = get_pending_mail(pending_address);
        if let Err(err) = res {
            continue;
        }
        let (author, pending) = res.unwrap();
        //  2. Convert and Commit as InMail
        let inmail = InMail::from_pending(pending, author);
        let inmail_entry = Entry::App("inmail".into(), inmail.into());
        let maybe_inmail_address = hdk::commit_entry(&inmail_entry);
        if maybe_inmail_address.is_err() {
            hdk::debug("Failed committing inMail");
            continue;
        }
        new_inmails.push(maybe_inmail_address.unwrap());
        //  3. Remove link from my agentId
        let res = hdk::remove_link(
            &AGENT_ADDRESS,
            &pending_address,
            "mail_inbox",
            LinkMatch::Any,
        );
        if let Err(err) = res {
            hdk::debug("Remove ``mail_inbox`` link failed:");
            hdk::debug(err);
            continue;
        }
        //  4. Delete PendingMail entry
        let res = hdk::remove_entry(pending_address);
        if let Err(err) = res {
            hdk::debug("Delete PendingMail failed:");
            hdk::debug(err);
            continue;
        }
    }
    Ok(new_inmails)
}