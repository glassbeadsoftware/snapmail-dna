use hdk::prelude::*;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
};
use crate::{
    entry_kind,
    FILE_MAX_SIZE,
};
use holochain_wasm_utils::{
    holochain_persistence_api::hash::HashString,
};

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing a file in chunks.
/// All chunks must be committed beforehand.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FileManifest {
    pub data_hash: HashString,
    pub filename: String,
    pub filetype: String,
    pub orig_filesize: usize,
    pub chunks: Vec<Address>,
}

pub fn file_manifest_def() -> ValidatingEntryType {
    entry!(
        name: entry_kind::FileManifest,
        description: "Entry for a file attachment manifest",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<FileManifest>| {
                validate_file(validation_data)
         }
    )
}

pub(crate) fn validate_file(validation_data: hdk::EntryValidationData<FileManifest>) -> Result<(), String> {
    // FIXME: Check if data_hash not already stored in source chain
    match validation_data {
        EntryValidationData::Create{entry: file, validation_data: _} => {
            // Check size
            if file.orig_filesize > FILE_MAX_SIZE {
                return Err(format!("A file can't be bigger than {} MiB", FILE_MAX_SIZE / (1024 * 1024)));
            }
            if file.orig_filesize < 1 {
                return Err("A file cannot be empty".into());
            }
            if file.chunks.len() < 1 {
                return Err("A file must have at least one chunk".into());
            }
            return Ok(());
        },
        EntryValidationData::Modify{new_entry: _, old_entry: _, old_entry_header:_, validation_data: _} => {
            return Err("Update file not allowed".into());
        },
        EntryValidationData::Delete{old_entry: _, old_entry_header: _, validation_data:_} => {
            return Ok(());
        }
    }
}




