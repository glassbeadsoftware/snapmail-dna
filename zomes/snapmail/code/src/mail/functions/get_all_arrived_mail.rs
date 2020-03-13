
/// Zome Function
/// Return list of all InMails that this agent did not acknowledge.
pub fn get_all_arrived_mail() -> ZomeApiResult<Vec<Address>> {
    // FIXME
    // 1. Get all InMails with query
    let result = hdk::query("inmail".into(),
                            0, 0)?;
    // For each InMail
    let mut unreads = Vec::new();
    for inmail_address in &result {
        //   2. Get Acknowledgment private link
        let res_count = hdk::get_links_count(inmail_address, "receipt_private".into(), LinkMatch::Any)?;
        //      b. if true continue
        if res.count > 0 {
            continue;
        }
        //   3. Get Acknowledgment encrypted link
        let res_count = hdk::get_links_count(inmail_address, "receipt_encrypted".into(), LinkMatch::Any)?;
        //      b. if true continue
        if res.count > 0 {
            continue;
        }
        //   4. Add to result list
        unreads.push(inmail_address.clone());
    }
    Ok(unreads)
}