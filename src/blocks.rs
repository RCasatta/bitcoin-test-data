const MAINNET_BLOCK_702861: &[u8; 1_381_836] =
    include_bytes!("../test_data/mainnet_block_702861.raw");

pub fn mainnet_702861() -> &'static [u8] {
    MAINNET_BLOCK_702861
}

#[cfg(test)]
mod tests {

    use electrsd::electrum_client::bitcoin::{
        consensus::deserialize,
        hashes::{hex::ToHex, sha256, Hash},
        Block,
    };

    use super::*;

    #[test]
    fn test_blocks() {
        let b: Block = deserialize(mainnet_702861()).unwrap();
        assert_eq!(
            b.block_hash().to_hex(),
            "000000000000000000000c835b2adcaedc20fdf6ee440009c249452c726dafae"
        );
        assert_eq!(
            sha256::Hash::hash(mainnet_702861()).to_hex(),
            "0fae3a62075a705aabac9cf063250fae07a461065157500828c1c4721a92fb5a"
        );
    }
}
