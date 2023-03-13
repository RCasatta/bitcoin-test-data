const SCRIPT_STATUS: &str = include_str!("../test_data/electrum_script_status.json");

pub fn script_status() -> &'static str {
    SCRIPT_STATUS
}

#[cfg(test)]
mod tests {

    use electrsd::{
        bitcoind::{self, bitcoincore_rpc::RpcApi, BitcoinD, P2P},
        electrum_client::{
            bitcoin::{hashes::hex::ToHex, Address, Amount},
            ElectrumApi,
        },
        ElectrsD,
    };
    use serde::{Deserialize, Serialize};

    use crate::tests::TestData;

    #[derive(Serialize, Deserialize)]
    pub struct Tx {
        txid: String,
        height: i32,
    }

    #[ignore]
    #[test]
    fn create_test_vectors() {
        let exe = bitcoind::downloaded_exe_path().unwrap();
        let mut conf = bitcoind::Conf::default();
        conf.p2p = P2P::Yes;
        let bitcoind = BitcoinD::with_conf(&exe, &conf).unwrap();

        let exe = electrsd::downloaded_exe_path().unwrap();
        let electrsd = ElectrsD::new(&exe, &bitcoind).unwrap();
        let mut result = vec![];

        let address = bitcoind.client.get_new_address(None, None).unwrap();
        let address_accumulate = bitcoind.client.get_new_address(None, None).unwrap();

        for addr in [&address, &address_accumulate] {
            electrsd
                .client
                .script_subscribe(&addr.script_pubkey())
                .unwrap();
        }

        let _ = bitcoind.client.generate_to_address(1, &address).unwrap();
        let _ = bitcoind
            .client
            .generate_to_address(100, &address_accumulate)
            .unwrap();

        electrsd.wait_height(102);

        let (script_history, status) = ask_history_and_status(&electrsd, &address);
        let test_data1 = make_test_data(script_history, status);
        result.push(test_data1);

        let txid = send_sat(&bitcoind, &address);
        electrsd.wait_tx(&txid);

        let (script_history, status) = ask_history_and_status(&electrsd, &address);
        let test_data2 = make_test_data(script_history, status);
        result.push(test_data2);

        let _blocks = bitcoind
            .client
            .generate_to_address(1, &address_accumulate)
            .unwrap();

        electrsd.wait_height(103);

        let (script_history, status) = ask_history_and_status(&electrsd, &address);
        let test_data3 = make_test_data(script_history, status);
        result.push(test_data3);

        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }

    fn send_sat(
        bitcoind: &BitcoinD,
        address: &Address,
    ) -> electrsd::electrum_client::bitcoin::Txid {
        bitcoind
            .client
            .send_to_address(
                &address,
                Amount::from_sat(10000),
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .unwrap()
    }

    fn make_test_data(
        script_history: Vec<electrsd::electrum_client::GetHistoryRes>,
        status: Option<electrsd::electrum_client::Hex32Bytes>,
    ) -> TestData<Vec<Tx>, String> {
        TestData {
            input: script_history
                .iter()
                .map(|h| Tx {
                    txid: h.tx_hash.to_string(),
                    height: h.height,
                })
                .collect::<Vec<_>>(),
            expected: status.unwrap().to_hex(),
        }
    }

    /// it's caller responsability to have already subsribed and receive an update
    fn ask_history_and_status(
        electrsd: &ElectrsD,
        address: &Address,
    ) -> (
        Vec<electrsd::electrum_client::GetHistoryRes>,
        Option<electrsd::electrum_client::Hex32Bytes>,
    ) {
        let script_history = electrsd
            .client
            .script_get_history(&address.script_pubkey())
            .unwrap();
        dbg!(&script_history);
        let status = electrsd
            .client
            .script_pop(&address.script_pubkey())
            .unwrap();
        (script_history, status)
    }
}
