//use hdk::prelude::*;

use std::time::SystemTime;
use std::convert::TryFrom;

use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::entry::{AppEntryValue, Entry},
};

/// Returns number of seconds since UNIX_EPOCH
pub fn snapmail_now() -> u64 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time must not be before UNIX EPOCH");
    duration_since_epoch.as_secs()
}


///
/// Helper function for loading an entry and converting to a given type
///
pub fn into_typed<R: TryFrom<AppEntryValue>>(entry: Entry) -> ZomeApiResult<R> {
    match entry {
        Entry::App(_, entry_value) => R::try_from(entry_value).map_err(|_| {
            ZomeApiError::Internal(
                "Could not convert entry to requested type".to_string(),
            )
        }),
        _ => Err(ZomeApiError::Internal(
            "entry did not return an entry of type App".to_string(),
        )),
    }
}