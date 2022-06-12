use std::{future::Future, sync::Arc};

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::requests::Payload;
use crate::{net, requests::ResponseResult};

mod api;

const BRAWL_TOKEN: &str = "BRAWL_TOKEN";

#[derive(Debug, Clone)]
pub struct Bot {
    token: Arc<str>,
    api_url: Arc<reqwest::Url>,
    client: Client,
}

/// Constructors
impl Bot {
    pub fn new<S>(token: S) -> Self
    where
        S: Into<String>,
    {
        let client = net::default_reqwest_settings()
            .build()
            .expect("client creation");

        Self::with_client(token, client)
    }

    fn with_client<S>(token: S, client: Client) -> Self
    where
        S: Into<String>,
    {
        let token = Into::<String>::into(token).into();
        let api_url = Arc::new(
            reqwest::Url::parse(net::BRAWL_API_URL).expect("to parse default brawl stars API url"),
        );

        Self {
            token,
            api_url,
            client,
        }
    }

    pub fn from_env() -> Self {
        Self::from_env_with_client(crate::net::client_from_env())
    }

    fn from_env_with_client(client: Client) -> Self {
        Self::with_client(&get_env(BRAWL_TOKEN), client)
    }

    pub fn set_api_url(mut self, url: reqwest::Url) -> Self {
        self.api_url = Arc::new(url);
        self
    }
}

// Getters
impl Bot {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn api_url(&self) -> reqwest::Url {
        reqwest::Url::clone(&self.api_url)
    }
}

impl Bot {
    pub(crate) fn execute_json<P>(
        &self,
        payload: &P,
    ) -> impl Future<Output = ResponseResult<P::Output>> + 'static
    where
        P: Payload + Serialize,
        P::Output: DeserializeOwned,
    {
        let client = self.client.clone();
        let token = Arc::clone(&self.token);
        let api_url = Arc::clone(&self.api_url);

        let params =
            serde_json::to_vec(payload).expect("serialization of request to be infallible");

        // async move to capture variables
        let name = payload.name();
        async move {
            net::request_json(
                &client,
                token.as_ref(),
                reqwest::Url::clone(&api_url),
                net::API_VERSION,
                &name,
                params,
            )
            .await
        }
    }
}

fn get_env(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable", env))
}
