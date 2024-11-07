use url::Url;

pub fn is_valid_long_url(url: &str) -> bool {
    url.len() <= 2048 && Url::parse(url).is_ok()
}

pub fn is_valid_title(title: &str) -> bool {
    title.len() <= 64
}

pub fn is_valid_description(description: &str) -> bool {
    description.len() <= 256
}
