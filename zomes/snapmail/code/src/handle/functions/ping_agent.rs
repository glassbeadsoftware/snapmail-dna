use hdk::{
    error::ZomeApiResult,
    holochain_core_types::time::Timeout,
};

use crate::{
    AgentAddress,
    protocol::DirectMessageProtocol,
};

/// Zome function
/// Return true if agent is online
pub fn ping_agent(destination: AgentAddress) -> ZomeApiResult<bool> {
    // 1. Send DM
    let payload = serde_json::to_string(&DirectMessageProtocol::Ping).unwrap();
    let result = hdk::send(
        destination.clone(),
        payload,
        Timeout::new(crate::DIRECT_SEND_TIMEOUT_MS),
    );
    hdk::debug(format!("ping result = {:?}", result)).ok();
    // 2. Check Response
    if let Ok(response) = result {
        hdk::debug(format!("ping response: {:?}", response)).ok();
        let maybe_msg: Result<DirectMessageProtocol, _> = serde_json::from_str(&response);
        if let Ok(msg) = maybe_msg {
            if let DirectMessageProtocol::Success(_) = msg {
                return Ok(true);
            }
        }
    };
    Ok(false)
}
