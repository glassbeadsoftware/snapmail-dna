use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
    },
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
};
use crate::{
    AgentAddress,
    mail::{
        OutMail, InMail,
    },
};
use syntax::util::map_in_place::MapInPlace;

//-------------------------------------------------------------------------------------------------
// Zome Function
//-------------------------------------------------------------------------------------------------

/// Zome function
/// Return address of newly created AckReceipt
pub fn mark_mail_as_read(inmail_address: &Address) -> ZomeApiResult<Address> {
    //  1. Make sure its an InMail
    let inmail = hdk::utils::get_as_type::<InMail>(inmail_address.clone())?;
    //  2. Make sure it has not already been acknowledged
    let res_count = hdk::get_links_count(inmail_address, "receipt_private".into(), LinkMatch::Any)?;
    if res.count > 0 {
        return Err(ZomeApiError::Internal("Mail has already been acknowledged (private)".to_string()));
    }
    let res_count = hdk::get_links_count(inmail_address, "receipt_encrypted".into(), LinkMatch::Any)?;
    if res.count > 0 {
        return Err(ZomeApiError::Internal("Mail has already been acknowledged (encrypted)".to_string()));
    }
    // 3. Try Direct Acknowledgment?
    // FIXME
    // 4. Acknowledge via DHT
    return mark_mail_as_read_encrypted(inmail_address, &inmail.from);
}

/// Return address of newly created AckReceiptPrivate
fn mark_mail_as_read_private(inmail_address: &Address) -> ZomeApiResult<Address> {
    // FIXME
}

/// Create & Commit AckReceiptEncrypted
/// Return address of newly created AckReceiptEncrypted
fn mark_mail_as_read_encrypted(inmail_address: &Address, from: &AgentAddress) -> ZomeApiResult<Address> {
    let ack = AckReceiptEncrypted::new(outmail_address.clone());
    let ack_entry = Entry::App("ackreceipt_encrypted".into(), ack.into());
    let ack_address = hdk::commit_entry(&ack_entry)?;
    let _ = hdk::link_entries(&inmail_address, &ack_address, "acknowledgment_encrypted", "")?;
    let _ = hdk::link_entries(&from, &ack_address, "ack_inbox", &HDK::AGENT_ADDRESS)?;
    Ok(ack_address)
}

/// Zome function
/// Check if agent received AckReceipts from all receipients of one of this agent's OutMail.
/// If false, returns list of agents who's receipt is missing.
/// TODO: Handle private Acks
pub fn have_received_all_receipts(outmail_address: &Address) -> ZomeApiResult<Result<(), Vec<AgentAddress>>> {
    // FIXME
    // 1. get OutMail
    let outmail = hdk::utils::get_as_type::<OutMail>(address)?;
    // 2. Merge all recepients lists into one
    let all_recepients = [outmail.mail.to, outmail.mail.cc, outmail.bcc].concat();
    // 3. get all ``receipt`` links and headers
    let links_result = hdk::get_links(outmail_address, LinkMatch::Exactly("receipt_encrypted"), LinkMatch::Any)?;
    // 4. Make list of Receipt authors
    let receipt_authors: Vec<AgentAddress> = links_result.tags().iter().map(|from_str| HashString::from(from_str))
    .collect();
    // 5. Diff lists
    let diff = all_recepients.iter().filter(|recepient| !receipt_authors.contains(recepient)).collect();
    Ok(if diff.size() > 0 {
       Ok(Err(diff))
    } else {
        Ok(())
    })
}

//-------------------------------------------------------------------------------------------------
// AckReceiptEncrypted
//-------------------------------------------------------------------------------------------------

/// Entry representing an AcknowldegmentReceipt on the DHT waiting to be received
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct AckReceiptEncrypted {
    outmail_address: Address,
}

pub fn ackreceipt_encrypted_def() -> ValidatingEntryType {
    entry!(
        name: "ackreceipt_encrypted",
        description: "Entry for an Acknowledgement Receipt of a Mail to be stored on the DHT",
        sharing: Sharing::Encrypted,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<AckReceiptEncrypted>| {
            // FIXME
            Ok(())
        },
        links: [
            from!(
                "%agent_id",
                link_type: "ack_inbox",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    // FIXME
                    Ok(())
                }
            ),
        ],
    )
}

impl AckReceiptEncrypted {
    pub fn new(outmail_address: Address) -> Self {
        Self {
            outmail_address,
        }
    }
}


//-------------------------------------------------------------------------------------------------
// AckReceiptPrivate
//-------------------------------------------------------------------------------------------------

/// Entry representing an AcknowldegmentReceipt private to to the agent receiving the Mail
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct AckReceiptPrivate {
}

pub fn ackreceipt_private_def() -> ValidatingEntryType {
    entry!(
        name: "ackreceipt_private",
        description: "Entry for an Acknowledgement Receipt of a Mail to stay private on source chain",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<AckReceiptPrivate>| {
            Ok(())
        }
    )
}

impl AckReceiptPrivate {
    pub fn new() -> Self {
        Self {
        }
    }
}