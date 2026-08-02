use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostContainer {
    pub post: Post,
}

impl From<Post> for PostContainer {
    fn from(post: Post) -> Self {
        Self { post }
    }
}
