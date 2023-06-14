use reqwest::header;
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;

use crate::resources::{Account, AccountResponse};

pub trait RequestConfig {
    fn path(&self, path: &str) -> String;
    fn cluster_url(&self) -> String;
    fn shop_language(&self) -> String;
    fn key(&self) -> String;
    fn secret(&self) -> Option<String>;
}

pub struct ClientConfig {
    key: String,
    secret: String,
    cluster: Cluster,
    language: Language,
}

impl RequestConfig for ClientConfig {
    fn path(&self, path: &str) -> String {
        let mut url = self.to_string();
        url.push_str(path);
        url
    }

    fn cluster_url(&self) -> String {
        self.cluster.to_string()
    }

    fn shop_language(&self) -> String {
        self.language.to_string()
    }

    fn key(&self) -> String {
       self.key.to_owned()
    }

    fn secret(&self) -> Option<String> {
        Some(self.secret.to_owned())
    }
}

impl ClientConfig {
    pub fn new(key: String, secret: String, cluster: Cluster, language: Language) -> Self {
        ClientConfig {
            key,
            secret,
            cluster,
            language,
        }
    }
}

impl ToString for ClientConfig {
    fn to_string(&self) -> String {
        format!("{cluster_url}{shop_language}/", cluster_url = &self.cluster_url(), shop_language = &self.shop_language())
    }
}

pub struct Client<S: RequestConfig> {
    client: reqwest::Client,
    config: S,
}

impl<S: RequestConfig> Client<S> {
    pub fn new(config: S) -> Result<Client<S>, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("User-Agent", HeaderValue::from_static("Lightspeed-API/0.0.1 (rust/2021)"));

        let http_client = reqwest::Client::builder().default_headers(headers).build()?;

        let client = Client {
            client: http_client,
            config,
        };
        Ok(client)
    }

    pub async fn account(&self) -> Result<Account, reqwest::Error> {
        let response = self.get::<AccountResponse>("/account.json").await?;
        Ok(response.account)
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, reqwest::Error> {
        let url = self.config.path(path);
        let response = self.client.get(url).basic_auth(&self.config.key(), self.config.secret()).send().await?.json().await?;
        Ok(response)
    }
}


pub enum Cluster {
    EU1,
    US1,
}

impl ToString for Cluster {
    fn to_string(&self) -> String {
        match &self {
            Cluster::US1 => "https://api.shoplightspeed.com/".into(),
            Cluster::EU1 => "https://api.webshopapp.com/".into()
        }
    }
}

pub enum Language {
    NL,
    EN,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match &self {
            Language::NL => "nl".into(),
            Language::EN => "en".into(),
        }
    }
}


#[cfg(test)]
mod tests {
    use httpmock::prelude::*;

    use crate::client::{Client, RequestConfig};

    struct MockServerConfig<'a> {
        server: &'a MockServer,
    }

    impl RequestConfig for MockServerConfig<'_> {
        fn path(&self, path: &str) -> String {
            self.server.url(path)
        }

        fn cluster_url(&self) -> String {
            "".to_string()
        }

        fn shop_language(&self) -> String {
            "".to_string()
        }

        fn key(&self) -> String {
            "".to_string()
        }

        fn secret(&self) -> Option<String> {
            Some("".to_string())
        }
    }

    #[tokio::test]
    async fn account_details() {
        let server = MockServer::start();

        let account_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/account.json")
                ;
            then.status(200)
                .header("content-type", "application/json")
                .body_from_file("tests/stubs/account.json");
        });

        let config = MockServerConfig {server: &server };
        if let Ok(client) = Client::new(config) {
            let details = client.account().await.unwrap();
            assert_eq!(details.id, 19609);
        }
        account_mock.assert();
    }
}