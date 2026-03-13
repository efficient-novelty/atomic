use blake3::Hash;

pub fn blake3_hash(bytes: &[u8]) -> Hash {
    blake3::hash(bytes)
}

pub fn blake3_hex(bytes: &[u8]) -> String {
    blake3_hash(bytes).to_hex().to_string()
}
