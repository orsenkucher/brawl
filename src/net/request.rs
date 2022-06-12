use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

use crate::{net::method_url, requests::ResponseResult, RequestError};

pub async fn request_json<T>(
    client: &Client,
    token: &str,
    api_url: reqwest::Url,
    api_version: &str,
    method_name: &str,
    params: Vec<u8>,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    let url = method_url(api_url, api_version, method_name);
    let request = client.get(url).bearer_auth(token).body(params).build()?;

    let response = client.execute(request).await?;

    // match response.error_for_status_ref() {
    //     Ok(_) => response.json().await.map_err(|err| err.into()),
    //     Err(_) => Err(RequestError::Api(response.json().await?)),
    // }
    process_response(response).await
}

async fn process_response<T>(response: Response) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    match response.error_for_status_ref() {
        Err(status) => {
            dbg!(status);
            Err(RequestError::Api(response.json().await?))
        }
        Ok(_) => {
            let text = response.text().await?;
            serde_json::from_str(&text)
                .map_err(|source| RequestError::InvalidJson {
                    source,
                    raw: text.into(),
                })
                .into()
        }
    }
}
