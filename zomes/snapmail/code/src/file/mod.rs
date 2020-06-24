use hdk::prelude::*;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address, hash::HashString,
    },
    holochain_core_types::{
        time::Timeout,
    },
};
//use multihash::{Hash};
use crate::{
    entry_kind,
};

// const CHUNK_MAX_SIZE: usize = 1024 * 500;
const CHUNK_MAX_SIZE: usize = 10;

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

// /// Entry representing a file. It is private.
// #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
// pub struct File {
//     pub full_data: String,
//     pub data_hash: HashString,
// }

/// Entry representing a file in chunks. It is private.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct File {
    pub data_hash: HashString,
    pub chunk_index: usize,
    pub chunk_total: usize,
    pub chunk: String,
}


pub fn file_def() -> ValidatingEntryType {
    entry!(
            name: entry_kind::File,
            description: "Entry for a file",
            sharing: Sharing::Private,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | validation_data: hdk::EntryValidationData<File>| {
                validate_file(validation_data)
            }
        )
}

pub(crate) fn validate_file(validation_data: hdk::EntryValidationData<File>) -> Result<(), String> {
    match validation_data {
        EntryValidationData::Create{entry: file, validation_data: _} => {
            // Check size
            if file.chunk.len() > CHUNK_MAX_SIZE {
                return Err("A file chunk can't be bigger than 500 KiB".into());
            }
            return Ok(());
        },
        EntryValidationData::Modify{new_entry: new_file, old_entry: old_file, old_entry_header:_, validation_data: _} => {
            // Check size
            if new_file.chunk.len() > CHUNK_MAX_SIZE {
                return Err("A file chunk can't be bigger than 500 KiB".into());
            }
            // Check invariants
            if new_file.data_hash != old_file.data_hash || new_file.chunk_total != old_file.chunk_total {
                return Err("Update chunk does not match file".into());
            }
            // // Check index
            // if new_file.chunk_index != old_file.chunk_index + 1 || new_file.chunk_index >= old_file.chunk_total {
            //     //if new_file.chunk_index < old_file.chunk_index + 1 || new_file.chunk_index >= old_file.chunk_total {
            //         return Err(format!("Incorrect chunk index. Total: {} ; old: {} ; new: {}"
            //                        , old_file.chunk_total, old_file.chunk_index, new_file.chunk_index).into());
            // }
            return Ok(());
        },
        EntryValidationData::Delete{old_entry: _, old_entry_header: _, validation_data:_} => {
            return Ok(());
        }
    }
}

impl File {
    // pub fn from_string(full_data: String) -> Self {
    //     let data_hash = HashString::encode_from_str(full_data.as_str(), Hash::SHA2256);
    //     Self {
    //         data_hash,
    //         full_data,
    //     }
    // }

    pub fn new(data_hash: HashString, chunk_index: usize, chunk_total: usize, chunk: String) -> Self {
        Self {
            data_hash,
            chunk_index,
            chunk_total,
            chunk,
        }
    }
}

// /// Split huge data_string to committable smaller strings
// fn split_chunks(data_string: String) -> Vec<&str> {
//     data_string.as_bytes()
//                      .chunks(CHUNK_MAX_SIZE)
//                      .map(|buf| unsafe { std::str::from_utf8_unchecked(buf) })
//                      .collect::<Vec<&str>>()
// }

// /// Zome function
// /// Write base64 file as string to source chain
// pub fn write_file(data_string: String) -> ZomeApiResult<Address> {
//     let orig_filesize = data_string.len();
//     hdk::debug(format!("orig_filesize = {}", orig_filesize)).ok();
//     let data_hash = HashString::encode_from_str(data_string.as_str(), Hash::SHA2256);
//
//     // split_chunks
//     // let subs = split_chunks(data_string);
//     let subs = data_string.as_bytes()
//                .chunks(CHUNK_MAX_SIZE)
//                .map(|buf| unsafe { std::str::from_utf8_unchecked(buf) })
//                .collect::<Vec<&str>>();
//
//
//     let chunk_total = subs.len();
//     // Create and commit initial chunk
//     let initial_file = File::new(data_hash.clone(), 0, chunk_total, subs[0].to_string());
//     let file_entry = Entry::App(entry_kind::File.into(), initial_file.into());
//     let maybe_file_address = hdk::commit_entry(&file_entry);
//     if let Err(e) = maybe_file_address {
//         let msg = format!("Failed committing initial file chunk: {:?}", e);
//         hdk::debug(msg.clone()).ok();
//         return Err(ZomeApiError::Internal(msg));
//     }
//     let initial_chunk_address = maybe_file_address.unwrap();
//     let mut previous_entry_address = initial_chunk_address.clone();
//     // Update subsequent chunks
//     for i in 1..chunk_total {
//         let update_file = File::new(data_hash.clone(), i, chunk_total, subs[i].to_string());
//         let file_entry = Entry::App(entry_kind::File.into(), update_file.into());
//         let maybe_file_address = hdk::update_entry(file_entry, &previous_entry_address);
//         if let Err(e) = maybe_file_address {
//             let msg = format!("Failed committing file chunk {} : {:?}", i, e);
//             hdk::debug(msg.clone()).ok();
//             return Err(ZomeApiError::Internal(msg));
//         }
//         previous_entry_address = maybe_file_address.unwrap();
//     }
//     hdk::debug(format!("First entry: {}", initial_chunk_address)).ok();
//     hdk::debug(format!("Last entry : {}", previous_entry_address)).ok();
//     Ok(initial_chunk_address)
// }


/// Zome function
/// Write base64 file as string to source chain
pub fn write_initial_chunk(data_hash: HashString, chunk_total: usize, first_chunk: String) -> ZomeApiResult<Address> {
    // Create and commit initial chunk
    let initial_file = File::new(data_hash.clone(), 0, chunk_total, first_chunk);
    let file_entry = Entry::App(entry_kind::File.into(), initial_file.into());
    let maybe_file_address = hdk::commit_entry(&file_entry);
    if let Err(e) = maybe_file_address {
        let msg = format!("Failed committing initial file chunk: {:?}", e);
        hdk::debug(msg.clone()).ok();
        return Err(ZomeApiError::Internal(msg));
    }
    let initial_chunk_address = maybe_file_address.unwrap();
    Ok(initial_chunk_address)
}

/// Zome function
/// Write base64 file as string to source chain
pub fn write_chunk(data_hash: HashString, chunk: String, initial_address: Address) -> ZomeApiResult<Address> {
    let _not_found_msg = "Initial file chunk not found";

    // // Get file
    // let maybe_entry = hdk::get_entry(&initial_address)?;
    // if let None = maybe_entry {
    //     hdk::debug(not_found_msg.clone()).ok();
    //     return Err(ZomeApiError::Internal(not_found_msg.to_string()));
    // }
    // //let file_address = initial_address.clone().into();
    // //let file_entry = maybe_entry.unwrap();
    //
    // // Check integrity
    // let latest_chunk = crate::into_typed::<File>(maybe_entry.unwrap()).expect("Should be File");
    // if latest_chunk.data_hash != data_hash {
    //     let err_msg = "File hash mismatch";
    //     hdk::debug(err_msg.clone()).ok();
    //     return Err(ZomeApiError::Internal(err_msg.to_string()));
    // }
    // let next_index = latest_chunk.chunk_index + 1;
    // if next_index == latest_chunk.chunk_total {
    //     let err_msg = "File already full";
    //     hdk::debug(err_msg.clone()).ok();
    //     return Err(ZomeApiError::Internal(err_msg.to_string()));
    // }

    // Update file
    let update_file = File::new(data_hash.clone(), 1, 2, chunk);
    let file_entry = Entry::App(entry_kind::File.into(), update_file.into());
    let maybe_file_address = hdk::update_entry(file_entry, &initial_address);
    if let Err(e) = maybe_file_address {
        let msg = format!("Failed committing file chunk {} : {:?}", 1, e);
        hdk::debug(msg.clone()).ok();
        return Err(ZomeApiError::Internal(msg));
    }
    let latest_entry_address = maybe_file_address.unwrap();

    hdk::debug(format!("First entry: {}", initial_address)).ok();
    hdk::debug(format!("Last entry : {}", latest_entry_address)).ok();
    Ok(latest_entry_address)
}
//
// /// Zome function
// /// Get File as base64 string in local source chain at address
// pub fn get_file(address: Address) -> Option<String> {
//
//     hdk::debug(format!("get_file(): {}", address)).ok();
//
//     // -- with get_entry_history()
//     let history_result = hdk::get_entry_history(&address);
//     if let Err(_e) = history_result {
//         hdk::debug("get_entry_history() failed").ok();
//         return None;
//     }
//     let maybe_file_history = history_result.unwrap();
//     if maybe_file_history.is_none() {
//         hdk::debug("No history found for File").ok();
//         return None;
//     }
//     let history = maybe_file_history.unwrap();
//     hdk::debug(format!("History length: {}", history.items.len())).ok();
//     hdk::debug(format!("History crud_links length: {}", history.crud_links.len())).ok();
//     let entry_items = history.items.clone();
//
//     // // -- with get_entry_result()
//     // let get_options = GetEntryOptions {
//     //     status_request: StatusRequestKind::All,
//     //     entry: true,
//     //     headers: true,
//     //     timeout: Timeout::default(),
//     // };
//     // let maybe_entry_result = hdk::get_entry_result(&address, get_options);
//     // if let Err(_e) = maybe_entry_result {
//     //     hdk::debug("get_entry_result() failed").ok();
//     //     return None;
//     // }
//     // let entry_result = maybe_entry_result.unwrap();
//     // let entry_items = match entry_result.result {
//     //     GetEntryResultType::Single(item) => {
//     //         let mut vec = Vec::new();
//     //         vec.push(item);
//     //         vec
//     //     },
//     //     GetEntryResultType::All(history) => {
//     //         history.items
//     //     },
//     // };
//     // // --
//
//     hdk::debug(format!("History length: {}", entry_items.len())).ok();
//
//     // //
//     // let item_iter = history.items.iter();
//     // let initial_file_entry = item_iter.entry.expect("should have entry");
//     // let initial_file = crate::into_typed::<File>(initial_file_entry).expect("Should be File");
//     //let mut full_data = String::with_capacity(initial_file.chunk_total * CHUNK_MAX_SIZE);
//
//     let mut full_data = String::new();
//
//     for item in entry_items {
//         let file_entry = item.entry.expect("should have entry");
//         //hdk::debug(format!("file_entry: {}", item.headers.len())).ok();
//         hdk::debug(format!("History headers length: {}", item.headers.len())).ok();
//         hdk::debug(format!("item crud status: {:?}", item.meta.unwrap().crud_status)).ok();
//         let file_chunk = crate::into_typed::<File>(file_entry).expect("Should be File");
//         // full_data.push_str(&file_chunk.chunk);
//         full_data = format!("{}{}", full_data, file_chunk.chunk);
//         hdk::debug(format!("chunk index: {}", file_chunk.chunk_index)).ok();
//     }
//     hdk::debug(format!("retrieved size = {}", full_data.len())).ok();
//     Some(full_data)
// }

/// Zome function
/// Get File as base64 string in local source chain at address
pub fn get_file(initial_address: Address, index: usize) -> ZomeApiResult<String> {

    hdk::debug(format!("get_file({}): {}", index, initial_address)).ok();

    // // -- with get_entry_history()
    // let history_result = hdk::get_entry_history(&initial_address);
    // if let Err(_e) = history_result {
    //     hdk::debug("get_entry_history() failed").ok();
    //     return Err(ZomeApiError::Internal("get_entry_history() failed".to_string()));
    // }
    // let maybe_file_history = history_result.unwrap();
    // if maybe_file_history.is_none() {
    //     hdk::debug("No history found for File").ok();
    //     return Err(ZomeApiError::Internal("No history found for File".to_string()));
    // }
    // let history = maybe_file_history.unwrap();
    // hdk::debug(format!("History length: {}", history.items.len())).ok();
    // hdk::debug(format!("History crud_links length: {}", history.crud_links.len())).ok();
    // let entry_items = history.items.clone();

    // -- with get_entry_result()
    let get_options = GetEntryOptions {
        status_request: StatusRequestKind::All,
        entry: false,
        headers: true,
        timeout: Timeout::default(),
    };
    let maybe_entry_result = hdk::get_entry_result(&initial_address, get_options);
    if let Err(_e) = maybe_entry_result {
        let msg = "get_entry_result() failed";
        hdk::debug(msg.clone()).ok();
        return Err(ZomeApiError::Internal(msg.to_string()));
    }
    let entry_result = maybe_entry_result.unwrap();
    hdk::debug(format!("entry_result: {:?}", entry_result)).ok();
    let entry_items = match entry_result.result {
        GetEntryResultType::Single(item) => {
            let mut vec = Vec::new();
            vec.push(item);
            vec
        },
        GetEntryResultType::All(history) => {
            history.items
        },
    };
    // --

    hdk::debug(format!("History length: {}", entry_items.len())).ok();
    if entry_items.len() <= index {
        let msg = "Chunk index higher than History length";
        hdk::debug(msg.clone()).ok();
        return Err(ZomeApiError::Internal(msg.to_string()));
    }
    // get indexed entry
    let maybe_nth_entry = hdk::get_entry_initial(entry_items[index].headers[0].entry_address())?;
    if maybe_nth_entry.is_none() {
        let msg = format!("File chunk {} not found", index);
        hdk::debug(msg.clone()).ok();
        return Err(ZomeApiError::Internal(msg));
    }
    let nth_entry = maybe_nth_entry.unwrap();

    // Check integrity
    let nth_chunk = crate::into_typed::<File>(nth_entry).expect("Should be File");
    if nth_chunk.chunk_index != index {
        let err_msg = "Chunk index mismatch";
        hdk::debug(err_msg.clone()).ok();
        return Err(ZomeApiError::Internal(err_msg.to_string()));
    }
    return Ok(nth_chunk.chunk);
}
