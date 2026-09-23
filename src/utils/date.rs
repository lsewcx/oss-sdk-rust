use chrono::Utc;

pub fn date_now() -> String {
    let now = Utc::now();
    return now.format("%Y%m%d").to_string();
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
