use std::collections::BTreeMap;

use super::Oss;

impl Oss {
    /// ListBuckets（GET /），服务级请求，常用于验证 AK/SK 和签名链路
    pub async fn list_buckets(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("https://{}/", self.endpoint);
        self.send(reqwest::Method::GET, "/", &url, BTreeMap::new(), None)
            .await
    }
}
