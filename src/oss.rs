use std::collections::BTreeMap;

use crate::utils::auth;

/// OSS 客户端：持有 AK/SK、region、endpoint 和复用的 HTTP 连接池
pub struct Oss {
    access_key_id: String,
    access_key_secret: String,
    region: String,
    endpoint: String,
    http: reqwest::Client,
}

impl Oss {
    /// 默认使用公网 endpoint：oss-{region}.aliyuncs.com
    pub fn new(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
        region: impl Into<String>,
    ) -> Self {
        let region = region.into();
        Self {
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            endpoint: format!("oss-{region}.aliyuncs.com"),
            region,
            http: reqwest::Client::new(),
        }
    }

    /// 覆盖 endpoint（VPC、CNAME、自定义域名等）
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    /// ListBuckets（GET /），服务级请求，常用于验证 AK/SK 和签名链路
    pub async fn list_buckets(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("https://{}/", self.endpoint);
        self.send(reqwest::Method::GET, "/", &url, BTreeMap::new(), None)
            .await
    }

    /// 通用签名请求：url 决定实际发往哪，canonical_path 始终按 /{bucket}/{key} 规则传
    async fn send(
        &self,
        method: reqwest::Method,
        canonical_path: &str,
        url: &str,
        headers: BTreeMap<String, String>,
        body: Option<String>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let (authorization, signed_headers) = auth::sign(
            &self.access_key_id,
            &self.access_key_secret,
            &self.region,
            method.as_str(),
            canonical_path,
            "",
            &[],
            headers,
        );

        let mut request = self
            .http
            .request(method, url)
            .header("Authorization", authorization);
        for (name, value) in &signed_headers {
            request = request.header(name, value);
        }
        if let Some(body) = body {
            request = request.body(body);
        }
        request.send().await
    }
}
