use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

type HmacSha256 = Hmac<Sha256>;

/// 字节转小写十六进制字符串
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// HMAC-SHA256，返回原始字节（用于签名密钥链）
fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts any key size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// SHA-256 摘要的小写十六进制
pub fn sha256_hex(data: &[u8]) -> String {
    hex(&Sha256::digest(data))
}

/// 六段拼 CanonicalRequest：method\npath\nquery\nheaders\nadditional\npayload
pub fn build_canonical_request(
    method: &str,
    path: &str,                                // "/"
    query: &str,                               // "" 没有就空串
    signed_headers: &BTreeMap<String, String>, // 只放参与签名的头（x-oss-*、content-type、content-md5）
    additional: &[String],
    payload: &str, // "UNSIGNED-PAYLOAD"
) -> String {
    let headers = signed_headers
        .iter()
        .map(|(k, v)| format!("{}:{}\n", k.to_lowercase(), v.trim()))
        .collect::<String>();
    let additional = additional.join(";");
    format!("{method}\n{path}\n{query}\n{headers}\n{additional}\n{payload}")
}

pub fn build_string_to_sign(datetime: &str, region: &str, canonical_request: &str) -> String {
    let date = &datetime[..8];
    let scope = format!("{date}/{region}/oss/aliyun_v4_request");
    format!(
        "OSS4-HMAC-SHA256\n{datetime}\n{scope}\n{}",
        sha256_hex(canonical_request.as_bytes())
    )
}

/// 派生 SigningKey 并对 StringToSign 签名，返回小写十六进制签名
pub fn build_signing_key(
    access_key_secret: &str,
    date: &str,
    region: &str,
    string_to_sign: &str,
) -> String {
    let date_key = hmac_sha256(
        format!("aliyun_v4{access_key_secret}").as_bytes(),
        date.as_bytes(),
    );
    let region_key = hmac_sha256(&date_key, region.as_bytes());
    let service_key = hmac_sha256(&region_key, b"oss");
    let signing_key = hmac_sha256(&service_key, b"aliyun_v4_request");
    hex(&hmac_sha256(&signing_key, string_to_sign.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_request_matches_official_vector() {
        let mut headers = BTreeMap::new();
        headers.insert("content-disposition".into(), "attachment".into());
        headers.insert("content-length".into(), "3".into());
        headers.insert("content-md5".into(), "ICy5YqxZB1uWSwcVLSNLcA==".into());
        headers.insert("content-type".into(), "text/plain".into());
        headers.insert("x-oss-content-sha256".into(), "UNSIGNED-PAYLOAD".into());
        headers.insert("x-oss-date".into(), "20250411T064124Z".into());

        let canonical_request = build_canonical_request(
            "PUT",
            "/examplebucket/exampleobject",
            "",
            &headers,
            &[
                "content-disposition".to_string(),
                "content-length".to_string(),
            ],
            "UNSIGNED-PAYLOAD",
        );

        assert_eq!(
            sha256_hex(canonical_request.as_bytes()),
            "c46d96390bdbc2d739ac9363293ae9d710b14e48081fcb22cd8ad54b63136eca"
        );
    }

    #[test]
    fn signature_matches_known_vector() {
        let signature = build_signing_key("testsecret", "20250417", "cn-hangzhou", "hello");
        assert_eq!(
            signature,
            "c203451f40f8d6fbb8b82fbeae1e69657f98ac791eec64a8afe3aa9d742b51cf"
        );
    }
}
