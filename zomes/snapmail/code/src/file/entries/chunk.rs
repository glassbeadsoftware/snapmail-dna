use hdk::prelude::*;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        hash::HashString,
    },
};
use crate::{
    entry_kind,
    CHUNK_MAX_SIZE,
};

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
