use argon2::password_hash::rand_core::{OsRng, RngCore as _};

/// Returns the current time in microseconds.
pub fn current_time_micros() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros() as i64
}
