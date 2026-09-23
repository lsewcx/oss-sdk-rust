use chrono::Utc;

pub fn date_now() -> String {
    Utc::now().format("%Y%m%d").to_string()
}

pub fn datetime_now() -> String {
    chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string() // 20250417T031540Z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_is_yyyymmdd() {
        let d = date_now();
        assert_eq!(d.len(), 8);
        assert!(d.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn date_now_matches_today() {
        let expected = Utc::now().format("%Y%m%d").to_string();
        assert_eq!(date_now(), expected);
    }
}
