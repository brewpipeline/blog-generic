use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct NewPostPublished {
    pub blog_user_id: u64,
    pub post_sub_url: String,
}

impl NewPostPublished {
    pub fn absolute_url(&self, site_url: &str) -> String {
        format!("{site_url}{path}", path = self.post_sub_url)
    }
}
