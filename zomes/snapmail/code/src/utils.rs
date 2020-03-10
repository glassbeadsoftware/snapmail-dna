use std::time::SystemTime;

pub fn snapmail_now() -> u64 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time must not be before UNIX EPOCH");
    duration_since_epoch.as_secs()
}
