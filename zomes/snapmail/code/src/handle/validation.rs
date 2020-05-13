use hdk::prelude::*;
use crate::handle::Handle;

/// Validates the handle's name field
fn validate_name(name: String) -> Result<(), String> {
    // FIXME: check name with regex
    // Check: min & max character count
    if name.len() < 3 {
        return Err("Name too short".into());
    }
    if name.len() > 32 {
        return Err("Name too long".into());
    }
    Ok(())
}

pub(crate) fn validate_handle(validation_data: hdk::EntryValidationData<Handle>) -> Result<(), String> {
    match validation_data {
        EntryValidationData::Create{entry: handle, validation_data: _} => {
            // FIXME: Check if author has already created a handle
            return validate_name(handle.name);
        },
        EntryValidationData::Modify{new_entry: new_handle, old_entry: old_handle, old_entry_header:_, validation_data: _} => {
        if new_handle.name == old_handle.name {
            return Err("Trying to modify with same data".into());
        }
            return validate_name(new_handle.name);
        },
            EntryValidationData::Delete{old_entry: _, old_entry_header: _, validation_data:_} => {
            return Err("Agent must have a Handle".into());
        }
    }
}