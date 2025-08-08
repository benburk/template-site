/// Generates a random 128-bit hex string.
pub fn generate_hex_token() -> String {
    format!("{:#018x}", fastrand::u128(..))
}
