use serde::de::DeserializeOwned;
use crate::traits::FromBytes;

#[derive(Debug)]
pub struct Json<T>(pub T);

impl<T: DeserializeOwned> FromBytes for Json<T> {
    type Error = serde_json::Error;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        let inner: T = serde_json::from_slice(bytes)?;
        Ok(Self(inner))
    }
}

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};
    use serde_json::json;
    use fluvio_dataplane_protocol::record::Record;
    use crate::record::Value;
    use crate::traits::FromRecord;
    use super::*;

    #[test]
    fn test_value_from_record() {
        let value = serde_json::to_string(&json!({ "message": "Hello world" })).unwrap();
        let record = Record::new(value);

        #[derive(Debug, Serialize, Deserialize)]
        struct Item {
            message: String,
        }

        fn get_that_value(Value(Json(item)): Value<Json<Item>>) {
            println!("Got: {}", item.message);
            assert_eq!(item.message, "Hello world");
        }

        let arg = match FromRecord::from_record(&record) {
            Ok(inner) => inner,
            Err(e) => panic!("failed: {}", e),
        };
        get_that_value(arg);
    }
}
