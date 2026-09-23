pub fn build_authorization(
    access_key_id: &str,
    sign_date: &str,
    sign_region: &str,
    additional_headers: &[String],
    signature: &str,
) -> String {
    let additional = if additional_headers.is_empty() {
        String::new()
    } else {
        format!(",AdditionalHeaders={}", additional_headers.join(";"))
    };

    format!(
        "OSS4-HMAC-SHA256 Credential={access_key_id}/{sign_date}/{sign_region}/oss/aliyun_v4_request{additional},Signature={signature}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn omits_additional_headers_when_empty() {
        let out = build_authorization("AK", "20250417", "cn-hangzhou", &[], "SIG");
        assert_eq!(
            out,
            "OSS4-HMAC-SHA256 Credential=AK/20250417/cn-hangzhou/oss/aliyun_v4_request,Signature=SIG"
        );
    }

    #[test]
    fn joins_additional_headers_with_semicolon() {
        let extra = vec!["host".to_string(), "content-length".to_string()];
        let out = build_authorization("AK", "20250417", "cn-hangzhou", &extra, "SIG");
        assert!(out.contains("AdditionalHeaders=host;content-length"));
    }
}
