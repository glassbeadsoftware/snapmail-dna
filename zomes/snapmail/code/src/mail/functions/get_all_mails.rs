use hdk::prelude::*;

use chrono::DateTime;
use std::convert::TryFrom;
use hdk::{
    error::ZomeApiResult,
    holochain_core_types::entry::{Entry},
};
use holochain_wasm_utils::{
    //holochain_core_types::link::LinkMatch,
    api_serialization::query::QueryArgsNames::QueryList,
};
use crate::{
    // link_kind,
    entry_kind,
    mail::entries::{*, self},
    mail::utils::{get_inmail_state, get_outmail_state},
};

/// Zome Function
/// Return list of all InMails and OutMails in the local source chain
pub fn get_all_mails() -> ZomeApiResult<Vec<MailItem>> {
    // 1. Get all mails with query
    let query_names = QueryList([entry_kind::InMail.to_owned(), entry_kind::OutMail.to_owned()].to_vec());
    let query_args = QueryArgsOptions {
        start: 0,
        limit: 0,
        headers: true,
        entries: true,
    };
    let query_result = hdk::query_result(query_names, query_args)?;
    hdk::debug(format!("query_result: {:?}", query_result)).ok();
    let mail_list = match query_result {
        QueryResult::HeadersWithEntries(list) => list,
        _ => panic!("Should be HeadersWithEntries"),
    };

    // For each mail
    let mut item_list = Vec::new();
    for (header, entry) in &mail_list {
        let date: i64 = DateTime::from(header.timestamp().clone()).timestamp_millis();
        let item = match entry {
            Entry::App(_, entry_value) => {
                if let Ok(inmail) = entries::InMail::try_from(entry_value.clone()) {
                    let state = MailState::In(get_inmail_state(header.entry_address()).expect("should be valid entry"));
                    let item = MailItem {
                        mail: inmail.mail,
                        state,
                        bcc: Vec::new(),
                        date,
                    };
                    item
                } else {
                    let outmail = entries::OutMail::try_from(entry_value).expect("Could not convert entry to requested type");
                    let state = MailState::Out(get_outmail_state(header.entry_address()).expect("should be valid entry"));

                    let item = MailItem {
                        mail: outmail.mail,
                        state,
                        bcc: outmail.bcc.clone(),
                        date,
                    };
                    item
                }
            },
            _ => panic!("Should be a mail Entry"),
        };
        // Add item to list
        item_list.push(item.clone());
    }
    Ok(item_list)
}