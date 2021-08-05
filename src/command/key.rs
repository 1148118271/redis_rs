use crate::redis_instance::RedisInstance;
use crate::{State, util};
use std::io::BufRead;
use std::io;

impl RedisInstance {

    /// 删除给定的一个key, 不存在的 key 会被忽略。
    ///```
    ///         let x = redis_rs::RedisClient::from("123456").connection().await.unwrap();
    ///         let state = x.delete("gxk").await.unwrap();
    ///         match state {
    ///             redis_rs::State::OK(v) => {
    ///                 println!("ok > {}", v)
    ///             }
    ///             redis_rs::State::ERROR(msg) => {
    ///                 println!("err > {}", msg)
    ///             }
    ///         }
    ///```
    pub async fn delete(&self, k: &str) -> Result<State<u32>, Box<dyn std::error::Error>> {
        let vec = self.assembly_and_send(&["DEL", k]).await?;
        let string = util::to_string(&vec[1..vec.len() - 2]);
        return if vec[0] as char == '-' {
            Ok(State::ERROR(string))
        } else {
            Ok(State::OK(string.parse::<u32>()?))
        }
    }

    /// 删除给定的一个或多个 key, 不存在的 key 会被忽略。
    ///```
    ///         let x = redis_rs::RedisClient::from("123456").connection().await.unwrap();
    ///         let state = x.delete_multiple(&["g1", "g2", "g3"]).await.unwrap();
    ///         match state {
    ///             redis_rs::State::OK(v) => {
    ///                 println!("ok > {}", v)
    ///             }
    ///             redis_rs::State::ERROR(msg) => {
    ///                 println!("err > {}", msg)
    ///             }
    ///         }
    ///```
    pub async fn delete_multiple(&self, ks: &[&str]) -> Result<State<u32>, Box<dyn std::error::Error>> {
        if ks.len() <= 0 {
            return Ok(State::OK(0))
        }
        let mut vec1 = ks.to_vec();
        let mut vec2 = vec!["DEL"];
        vec2.append(&mut vec1);
        drop(vec1);
        let vec = self.assembly_and_send(&vec2).await?;
        let string = util::to_string(&vec[1..vec.len() - 2]);
        return if vec[0] as char == '-' {
            Ok(State::ERROR(string))
        } else {
            Ok(State::OK(string.parse::<u32>()?))
        }
    }
}