
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
        // async_!(set_str());
        // async_!(get_str());
        async_!(append_str());
    }

    async fn set_str() {
        let x = redis_client::RedisClient::from("123456").connection().await.unwrap();
        // let state = x.set_string("aa", "").await.unwrap();
        let state = x.set_string("gxk", "gxk").await.unwrap();
        match state {
            State::OK(v) => {
                println!("ok > {}", v)
            }
            State::ERROR(msg) => {
                println!("err > {}", msg)
            }
        }
    }


    async fn get_str() {
        let x = redis_client::RedisClient::from("123456").connection().await.unwrap();
        let state = x.get_string("gxk").await.unwrap();
        match state {
            State::OK(v) => {
                match v {
                    None => println!("值为空"),
                    Some(s) => println!("查询成功 -> {}", s)
                }
            }
            State::ERROR(msg) => println!("err > {}", msg)
        }
    }

    async fn append_str() {
        let x = redis_client::RedisClient::from("123456").connection().await.unwrap();
        let state = x.append("166", "123").await.unwrap();
        match state {
            State::OK(v) => {
                println!("ok > {}", v)
            }
            State::ERROR(msg) => {
                println!("err > {}", msg)
            }
        }
    }
}