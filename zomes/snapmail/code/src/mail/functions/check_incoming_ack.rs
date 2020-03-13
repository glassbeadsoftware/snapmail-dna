
/// Return list of AckReceiptEncryted addresses
pub fn check_incoming_ack() -> ZomeApiResult<Vec<Address>> {
    // Lookup `ack_inbox` links on my agentId
    let links_result = hdk::get_links(&HDK::AGENT_ADDRESS, LinkMatch::Exactly("ack_inbox"), LinkMatch::Any)?;
    // For each link
    let mut new_acks = Vec::new();
    for ack_address in &links_result.addresses() {
        //  - Get entry on the DHT
        let res = get_ack_encrypted(ack_address);
        if let Err(err) = res {
            continue;
        }
        let (author, ack) = res.unwrap();
        //  - Add Acknowledgement link to my OutMail
        let res = hdk::link_entries(
            &HDK::AGENT_ADDRESS,
            &ack_address,
            "receipt_encrypted",
            author.into());
        if let Err(err) = res {
            hdk::debug("Add ``receipt_encrypted`` link failed:");
            hdk::debug(err);
            continue;
        }
        //  - Delete AckReceipt link from my agentId
        let res = hdk::remove_link(
            &AGENT_ADDRESS,
            &ack_address,
            "ack_inbox",
            LinkMatch::Any,
        );
        if let Err(err) = res {
            hdk::debug("Remove ``ack_inbox`` link failed:");
            hdk::debug(err);
            continue;
        }
    }
    Ok(new_acks)
}