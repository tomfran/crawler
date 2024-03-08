use url::Url;

pub fn extract_domain(url: &str) -> Option<String> {
    let url = Url::parse(url).ok()?;
    base_url(url).map(|u| u.to_string())
}

pub fn clean_url(url: &str) -> Option<String> {
    let mut url = Url::parse(url).ok()?;
    url.set_query(None);
    url.set_fragment(None);
    Some(url.to_string())
}

fn base_url(mut url: Url) -> Option<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(()) => {
            return None;
        }
    }
    url.set_query(None);
    url.set_fragment(None);
    Some(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            extract_domain("https://www.example.com/path?query=query"),
            Some("https://www.example.com/".to_string())
        );
        assert_eq!(
            extract_domain("https://example.com"),
            Some("https://example.com/".to_string())
        );
        assert_eq!(extract_domain("invalid-url"), None);
    }

    #[test]
    fn test_clean_url() {
        assert_eq!(
            clean_url("https://www.example.com/path?query=query"),
            Some("https://www.example.com/path".to_string())
        );
        assert_eq!(
            clean_url("http://example.com/#index"),
            Some("http://example.com/".to_string())
        );
        assert_eq!(clean_url("invalid-url"), None);
    }
}
