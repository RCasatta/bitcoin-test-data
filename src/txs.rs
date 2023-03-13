const MAINNET_73BE398C4BDC43709DB7398106609EEA2A7841AAF3A4FA2000DC18184FAA2A7E: &[u8; 500_142] =
    include_bytes!("../test_data/huge_witness.raw");

/// Transaction with a huge witness script, causing some bugs in production when happeread in block
/// 761249
pub fn mainnet_73be398c4bdc43709db7398106609eea2a7841aaf3a4fa2000dc18184faa2a7e() -> &'static [u8] {
    MAINNET_73BE398C4BDC43709DB7398106609EEA2A7841AAF3A4FA2000DC18184FAA2A7E
}

#[cfg(test)]
mod tests {
    use electrsd::electrum_client::bitcoin::{
        consensus::deserialize, hashes::hex::ToHex, Transaction,
    };

    #[test]
    fn test_txs2() {
        let tx: Transaction = deserialize(
            &super::mainnet_73be398c4bdc43709db7398106609eea2a7841aaf3a4fa2000dc18184faa2a7e(),
        )
        .unwrap();
        assert_eq!(
            tx.txid().to_hex(),
            "73be398c4bdc43709db7398106609eea2a7841aaf3a4fa2000dc18184faa2a7e"
        );
    }
}
