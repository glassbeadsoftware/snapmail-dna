use hdk::prelude::*;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address, hash::HashString,
    },
};
use crate::{
    entry_kind,
};


// const CHUNK_MAX_SIZE: usize = 1024 * 500;
const CHUNK_MAX_SIZE: usize = 200 * 1024;

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing a file chunk.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FileChunk {
    pub data_hash: HashString,
    pub chunk_index: usize,
    pub chunk: String,
}


pub fn file_chunk_def() -> ValidatingEntryType {
    entry!(
            name: entry_kind::FileChunk,
            description: "Entry for a file",
            sharing: Sharing::Private,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | validation_data: hdk::EntryValidationData<FileChunk>| {
                validate_chunk(validation_data)
            }
        )
}

pub(crate) fn validate_chunk(validation_data: hdk::EntryValidationData<FileChunk>) -> Result<(), String> {
    match validation_data {
        EntryValidationData::Create{entry: file, validation_data: _} => {
            // Check size
            if file.chunk.len() > CHUNK_MAX_SIZE {
                return Err(format!("A file chunk can't be bigger than {} KiB", CHUNK_MAX_SIZE / 1024));
            }
            return Ok(());
        },
        EntryValidationData::Modify{new_entry: _, old_entry: _, old_entry_header:_, validation_data: _} => {
            return Err("Update chunk not allowed".into());
        },
        EntryValidationData::Delete{old_entry: _, old_entry_header: _, validation_data:_} => {
            return Ok(());
        }
    }
}

impl FileChunk {
    pub fn new(data_hash: HashString, chunk_index: usize, chunk: String) -> Self {
        Self {
            data_hash,
            chunk_index,
            chunk,
        }
    }
}

/// Zome function
/// Write base64 file as string to source chain
pub fn write_chunk(
    data_hash: HashString,
    chunk_index: usize,
    chunk: String,
) -> ZomeApiResult<Address> {
    let initial_file = FileChunk::new(data_hash.clone(), chunk_index, chunk);
    let file_entry = Entry::App(entry_kind::FileChunk.into(), initial_file.into());
    let maybe_file_address = hdk::commit_entry(&file_entry);
    maybe_file_address
}

/// Zome function
/// Get chunk index and chunk as base64 string in local source chain at given address
pub fn get_chunk(chunk_address: Address) -> ZomeApiResult<String> {
    hdk::debug(format!("get_chunk(): {}", chunk_address)).ok();
    let maybe_entry = hdk::get_entry(&chunk_address)
        .expect("No reason for get_entry() to crash");
    if maybe_entry.is_none() {
        return Err(ZomeApiError::Internal("No chunk found at given address".into()))
    }
    let chunk = crate::into_typed::<FileChunk>(maybe_entry.unwrap())?;
    // Ok((chunk.chunk_index, chunk.chunk))
    Ok(chunk.chunk)
}
