pub mod types;
pub mod point_ops;
pub mod payload_ops;

use serde::{Deserialize, Serialize};
use schemars::{JsonSchema};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum CollectionUpdateOperations {
    PointOperation(point_ops::PointOps),
    PayloadOperation(payload_ops::PayloadOps),
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize() {
        let op = CollectionUpdateOperations::PayloadOperation(
            payload_ops::PayloadOps::ClearPayload {
                points: vec![1, 2, 3],
            }
        );

        let json = serde_json::to_string_pretty(&op).unwrap();
        println!("{}", json)
    }
}