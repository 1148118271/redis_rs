use std::default::Default;
use tokio::io;
use crate::tcp::Client;
use crate::redis_instance::Instance;

#[derive(Debug)]
pub struct RedisClient {
    host: String,
    port: u16,
    pass: Option<String>,
}

impl Default for RedisClient {
    fn default() -> Self {
        RedisClient {
            host: "127.0.0.1".to_string(),
            port: 6379,
            pass: None
        }
    }
}

impl RedisClient {
    pub fn new() -> Self {
        RedisClient::default()
    }

    pub async fn connection(self) -> io::Result<Instance> {
        let client = Client::connect(format!("{}:{}", self.host, self.port)).await?;
        let instance = Instance::init(client);
        match self.pass {
            Some(p) => {
                instance.login(&p).await?;
            }
            _ => {},
        }
        Ok(instance)
    }

}

impl From<(&str, u16, &str)> for RedisClient {
    fn from(data: (&str, u16, &str)) -> Self {
        let mut client = RedisClient::default();
        client.host = data.0.to_string();
        client.port = data.1;
        let pass = data.2.to_string();
        if pass == "" {
            client.pass = None
        }
        else {
            client.pass = Some(pass)
        }

        client
    }
}

impl From<(&str, u16)> for RedisClient {
    fn from(data: (&str, u16)) -> Self {
        let mut client = RedisClient::default();
        client.host = data.0.to_string();
        client.port = data.1;

        client
    }
}

impl From<&str> for RedisClient {
    fn from(pass: &str) -> Self {
        let mut client = RedisClient::default();
        let pass = pass.to_string();
        if pass.len() <= 0 {
            client.pass = None
        }
        else {
            client.pass = Some(pass)
        }

        client
    }
}

