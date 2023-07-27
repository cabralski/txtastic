// The entirety of Base62 (some characters aren't doable, such as: "/")
const BASE62: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

// "Utilities for random number generation."
use rand::{self, Rng};

/// This function generates a random `ID` for the `Txt`.
pub fn generate_id(size: usize) -> String {
    (0..size)
        .map(|_| BASE62[rand::thread_rng().gen_range(0..62)] as char)
        .collect()
}