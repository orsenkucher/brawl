use std::time::Duration;

pub(crate) use self::request::request_json;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

mod request;

// curl -H 'Authorization: Bearer API_KEY' https://api.brawlstars.com/v1/players/%23Q0CQ90Lq9/battlelog
pub const BRAWL_API_URL: &str = "https://api.brawlstars.com";
pub const API_VERSION: &str = "v1";

pub fn client_from_env() -> reqwest::Client {
    let builder = default_reqwest_settings();
    builder.build().expect("creating reqwest::Client")
}

pub fn default_reqwest_settings() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(17))
        .tcp_nodelay(true)
}

fn method_url(base: reqwest::Url, api_version: &str, method_name: &str) -> reqwest::Url {
    base.join(&format!(
        "/{}/{}",
        percent_encoding(api_version),
        percent_encoding(method_name)
    ))
    .expect("to format url")
}

fn percent_encoding(url: &str) -> String {
    ///https://url.spec.whatwg.org/#query-percent-encode-set
    const QUERY: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
    utf8_percent_encode(url, QUERY).to_string()
}

#[cfg(test)]
mod tests {
    use crate::net::*;

    #[test]
    fn method_url_test() {
        let url = method_url(
            reqwest::Url::parse(BRAWL_API_URL).unwrap(),
            API_VERSION,
            "players",
        );
        assert_eq!(url.as_str(), "https://api.brawlstars.com/v1/players")
    }
}
