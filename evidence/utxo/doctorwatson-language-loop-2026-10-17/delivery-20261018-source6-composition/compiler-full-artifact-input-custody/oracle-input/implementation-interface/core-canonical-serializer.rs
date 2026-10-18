pub fn canonical<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    fn sort(v: serde_json::Value) -> serde_json::Value {
        match v {
            serde_json::Value::Object(map) => {
                let sorted: std::collections::BTreeMap<_, _> =
                    map.into_iter().map(|(k, v)| (k, sort(v))).collect();
                serde_json::Value::Object(sorted.into_iter().collect())
            }
            serde_json::Value::Array(a) => {
                serde_json::Value::Array(a.into_iter().map(sort).collect())
            }
            v => v,
        }
    }
    let value = serde_json::to_value(value).map_err(|e| Error::new("intent", e.to_string()))?;
    serde_json::to_vec(&sort(value)).map_err(|e| Error::new("intent", e.to_string()))
}
pub fn hex(bytes: &[u8]) -> String {
