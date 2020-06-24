use hdk::prelude::*;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
};
use crate::{
    entry_kind,
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
    pub orig_filesize: u64,
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
        validation: | _validation_data: hdk::EntryValidationData<FileManifest>| {
            // FIXME: Check if data_hash not already stored in source chain
            Ok(())
        }
    )
}

/// Zome function
/// Write file manifest to source chain
pub fn write_manifest(
    data_hash: HashString,
    filename: String,
    filetype: String,
    orig_filesize: u64,
    chunks: Vec<Address>,
) -> ZomeApiResult<Address> {
    let manifest = FileManifest {
        data_hash, filename, filetype, orig_filesize, chunks
    };
    let file_entry = Entry::App(entry_kind::FileManifest.into(), manifest.into());
    let maybe_file_address = hdk::commit_entry(&file_entry);
    maybe_file_address
}

/// Zome function
/// Get manifest entry at given address
pub fn get_manifest(manifest_address: Address) -> ZomeApiResult<FileManifest> {
    hdk::debug(format!("get_manifest(): {}", manifest_address)).ok();
    let maybe_entry = hdk::get_entry(&manifest_address)
        .expect("No reason for get_entry() to crash");
    if maybe_entry.is_none() {
        return Err(ZomeApiError::Internal("No entry found at given address".into()))
    }
    let manifest = crate::into_typed::<FileManifest>(maybe_entry.unwrap())?;
    Ok(manifest)
}

/// Zome function
/// Get manifest entry at given address
pub fn find_manifest(data_hash: HashString) -> ZomeApiResult<Option<FileManifest>> {
    hdk::debug(format!("get_manifest(): {}", data_hash)).ok();
    let query_result = hdk::query(entry_kind::FileManifest.into(), 0, 0)?;
    // For each File chunk
    for manifest_address in &query_result {
        // Get entry
        let entry = hdk::get_entry(manifest_address)
            .expect("No reason for get_entry() to crash")
            .expect("Should have it");
        let manifest = crate::into_typed::<FileManifest>(entry).expect("Should be a FileManifest");
        if manifest.data_hash == data_hash {
            return Ok(Some(manifest));
        }
    }
    Ok(None)
}

/// Zome function
/// Get all manifests stored in our source chain
pub fn get_all_manifests() -> ZomeApiResult<Vec<FileManifest>> {
    hdk::debug(format!("get_all_manifests()")).ok();
    let query_result = hdk::query(entry_kind::FileManifest.into(), 0, 0)?;
    // For each File chunk
    let mut manifest_list = Vec::new();
    for manifest_address in &query_result {
        // Get entry
        let entry = hdk::get_entry(manifest_address)
            .expect("No reason for get_entry() to crash")
            .expect("Should have it");
        let manifest = crate::into_typed::<FileManifest>(entry).expect("Should be a FileManifest");
        // Add to list
        manifest_list.push(manifest);
    }
    Ok(manifest_list)
}