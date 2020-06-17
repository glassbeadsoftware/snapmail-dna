use hdk::prelude::*;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address, hash::HashString,
    },
};
use multihash::{Hash};
use crate::{
    entry_kind,
};

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing a file. It is private.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct File {
    pub full_data: String,
    pub data_hash: HashString,
}


pub fn file_def() -> ValidatingEntryType {
    entry!(
            name: entry_kind::File,
            description: "Entry for a file",
            sharing: Sharing::Private,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<File>| {
                // FIXME
                Ok(())
            }
        )
}

impl File {
    pub fn from_string(full_data: String) -> Self {
        let data_hash = HashString::encode_from_str(full_data.as_str(), Hash::SHA2256);
        Self {
            data_hash,
            full_data,
        }
    }
}

/// Zome function
/// Write base64 file as string to source chain
pub fn write_file(data_string: String) -> ZomeApiResult<Address> {
    let file = File::from_string(data_string);
    let file_entry = Entry::App(entry_kind::File.into(), file.into());
    let maybe_file_address = hdk::commit_entry(&file_entry);
    if maybe_file_address.is_err() {
        hdk::debug("Failed committing File").ok();
    }
    maybe_file_address
}

/// Zome function
/// Get File as base64 string in local source chain at address
pub fn get_file(address: Address) -> Option<String> {
    let maybe_file = hdk::utils::get_as_type::<File>(address.clone());
    if let Ok(file) = maybe_file {
        return Some(file.full_data);
    }
    None
}