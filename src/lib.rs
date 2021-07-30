
mod tcp;
mod redis_client;
mod redis_instance;
mod string;
mod result;
mod util;

pub use redis_client::RedisClient;

pub use result::State;

#[cfg(test)]
mod test {
    use std::io::BufRead;
    use crate::{redis_client, State};

    #[test]
    fn test1() {
        let a = "aa\r\nbb\r\n".as_bytes();
        let mut result = a.lines();
       // let mut split = a.split(|v| *v as char == '\r' || *v as char == '\n');
        println!("{:?}", result.next().unwrap());
        println!("{:?}", result.next().unwrap());

    }

    macro_rules! r#async_ {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }


    #[test]
    fn test2() {
        async_!(connection())
    }

    async fn connection() {
        let client = redis_client::RedisClient::from("123456");
        let instance = client.connection().await.unwrap();
        let result = instance.set_string("aaa", "123").await.unwrap();
        match result.state {
            State::OK => {
                println!("string set success -> {}", result.value);
            }
            State::ERROR => {
                println!("string set error -> {}", result.value);
            }
            State::NULL => {
                println!("result is null");
            }
        }
    }
}