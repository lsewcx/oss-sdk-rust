use std::collections::BTreeMap;

use crate::utils::{crypto, date::datetime_now};

pub const UNSIGNED_PAYLOAD: &str = "UNSIGNED-PAYLOAD";

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

/// 给请求头补上 x-oss-content-sha256、x-oss-date，计算并返回 (Authorization, 最终请求头)。
/// 调用方必须原样发送返回的这些头，否则服务端重算的签名会对不上。
#[allow(clippy::too_many_arguments)]
pub fn sign(
    access_key_id: &str,
    access_key_secret: &str,
    region: &str,
    method: &str,
    canonical_path: &str,
    query: &str,
    additional_headers: &[String],
    mut headers: BTreeMap<String, String>,
) -> (String, BTreeMap<String, String>) {
    let datetime = datetime_now();
    headers.insert(
        "x-oss-content-sha256".to_string(),
        UNSIGNED_PAYLOAD.to_string(),
    );
    headers.insert("x-oss-date".to_string(), datetime.clone());

    let canonical_request = crypto::build_canonical_request(
        method,
        canonical_path,
        query,
        &headers,
        additional_headers,
        UNSIGNED_PAYLOAD,
    );
    let string_to_sign = crypto::build_string_to_sign(&datetime, region, &canonical_request);
    let signature =
        crypto::build_signing_key(access_key_secret, &datetime[..8], region, &string_to_sign);
    let authorization = build_authorization(
        access_key_id,
        &datetime[..8],
        region,
        additional_headers,
        &signature,
    );

    (authorization, headers)
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

    #[test]
    fn sign_injects_required_headers() {
        let (authorization, signed) = sign(
            "AK",
            "SK",
            "cn-hangzhou",
            "GET",
            "/",
            "",
            &[],
            BTreeMap::new(),
        );
        assert!(authorization.starts_with("OSS4-HMAC-SHA256 Credential=AK/"));
        assert_eq!(
            signed.get("x-oss-content-sha256").map(String::as_str),
            Some(UNSIGNED_PAYLOAD)
        );
        assert!(signed.contains_key("x-oss-date"));
    }
}
