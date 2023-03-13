pub mod blocks;
pub mod electrum;
pub mod txs;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct TestData<I, E> {
        #[serde(bound(serialize = "I: Serialize", deserialize = "I: Deserialize<'de>"))]
        pub input: I,

        #[serde(bound(serialize = "E: Serialize", deserialize = "E: Deserialize<'de>"))]
        pub expected: E,
    }

    #[test]
    fn test_test_data() {
        let json = r#"
        [{"input":1,"expected":"1"}]
        "#;

        let data: Vec<TestData<u32, String>> = serde_json::from_str(&json).unwrap();
        assert_eq!(data[0].input, 1);
        assert_eq!(data[0].expected, "1");
    }
}
