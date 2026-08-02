use serde::{Deserialize, Serialize};
#[cfg(feature = "validator")]
use validator::Validate;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validator", derive(Validate))]
#[serde(rename_all = "camelCase")]
pub struct CommonComment {
    pub post_id: u64,
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 500)))]
    pub content: String,
}
