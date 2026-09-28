use std::collections::BTreeMap;

use super::Oss;

impl Oss {
    /// ListBuckets（GET /），服务级请求，常用于验证 AK/SK 和签名链路
    pub async fn list_buckets(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("https://{}/", self.endpoint);
        self.send(reqwest::Method::GET, "/", "", &url, BTreeMap::new(), None)
            .await
    }
    pub async fn get_bucket_acl(&self, bucket: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("https://{bucket}.{}/?acl", self.endpoint);
        self.send(
            reqwest::Method::GET,
            &format!("/{bucket}/"),
            "acl",
            &url,
            BTreeMap::new(),
            None,
        )
        .await
    }

    /// 判断桶是否存在，语义与官方 SDK 一致：
    /// - 请求成功 -> true
    /// - NoSuchBucket -> false
    /// - 其他服务端业务错误（AccessDenied、SignatureDoesNotMatch 等）-> true
    /// - 网络等非服务端错误 -> Err
    pub async fn is_bucket_exist(&self, bucket: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let resp = self.get_bucket_acl(bucket).await?;
        if resp.status().is_success() {
            return Ok(true);
        }
        let body = resp.text().await?;
        match extract_code(&body) {
            Some("NoSuchBucket") => Ok(false),
            Some(_) => Ok(true),
            None => Err(format!("unexpected OSS response: {body}").into()),
        }
    }
}

fn extract_code(body: &str) -> Option<&str> {
    let start = body.find("<Code>")? + "<Code>".len();
    let end = body[start..].find("</Code>")? + start;
    Some(&body[start..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_error_code_from_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Error>
  <Code>NoSuchBucket</Code>
  <Message>...</Message>
</Error>"#;
        assert_eq!(extract_code(xml), Some("NoSuchBucket"));
    }

    #[test]
    fn returns_none_when_code_missing() {
        assert_eq!(extract_code("<html>gateway error</html>"), None);
    }
}
