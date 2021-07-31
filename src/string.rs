use crate::redis_instance::RedisInstance;
use std::io;
use crate::util;
use crate::result::State;
use std::io::BufRead;
use std::num::ParseIntError;

impl RedisInstance {

    /// 将字符串值 value 关联到 key 。
    /// 如果 key 已经持有其他值， SET 就覆写旧值，无视类型。
    /// 对于某个原本带有生存时间（TTL）的键来说， 当 SET 命令成功在这个键上执行时， 这个键原有的 TTL 将被清除。
    /// 返回值：
    ///     在 Redis 2.6.12 版本以前， SET 命令总是返回 OK 。
    ///     从 Redis 2.6.12 版本开始， SET 在设置操作成功完成时，才返回 OK 。
    /// ```
    ///         let x = redis_rs::RedisClient::from("123456").connection().await.unwrap();
    ///         let state = x.set_string("gxk", "gxk").await.unwrap();
    ///         match state {
    ///             redis_rs::State::OK(v) => {
    ///                 println!("ok > {}", v)
    ///             }
    ///             redis_rs::State::ERROR(msg) => {
    ///                 println!("err > {}", msg)
    ///             }
    ///         }
    /// ```
    pub async fn set_string(&self, k: &str, v: &str) -> io::Result<State<String>> {
        if v.len() <= 0 {
            panic!("value 不能为空!")
        }
        let vec = self.assembly_and_send(&["SET", k, v]).await?;
        let msg = util::to_string(&vec[1..vec.len() - 2]);
        return if vec[0] as char == '-' {
            Ok(State::ERROR(msg))
        } else {
            Ok(State::OK(msg))
        }
    }

    /// 将字符串值 value 关联到 key 。
    /// 如果 key 已经持有其他值， SET 就覆写旧值，无视类型。
    /// 对于某个原本带有生存时间（TTL）的键来说， 当 SET 命令成功在这个键上执行时， 这个键原有的 TTL 将被清除。
    /// 返回值：
    ///     在 Redis 2.6.12 版本以前， SET 命令总是返回 OK 。
    ///     从 Redis 2.6.12 版本开始， SET 在设置操作成功完成时，才返回 OK 。
    /// EX second ：设置键的过期时间为 second 秒。
    /// # params : [`seconds`] 单位为`秒`
    /// ```
    ///         let x = redis_rs::RedisClient::from("123456").connection().await.unwrap();
    ///         let state = x.set_string_ex("gxk", "gxk", 50).await.unwrap();
    ///         match state {
    ///             redis_rs::State::OK(v) => {
    ///                 println!("ok > {}", v)
    ///             }
    ///             redis_rs::State::ERROR(msg) => {
    ///                 println!("err > {}", msg)
    ///             }
    ///         }
    /// ```
    pub async fn set_string_ex(&self,  k: &str, v: &str, seconds: usize) -> io::Result<State<String>> {
        if v.len() <= 0 {
            panic!("value 不能为空!")
        }
        let vec = self.assembly_and_send(&["SET", k, v, "EX", &seconds.to_string()]).await?;
        let msg = util::to_string(&vec[1..vec.len() - 2]);
        return if vec[0] as char == '-' {
            Ok(State::ERROR(msg))
        } else {
            Ok(State::OK(msg))
        }
    }

    /// 将字符串值 value 关联到 key 。
    /// 如果 key 已经持有其他值， SET 就覆写旧值，无视类型。
    /// 对于某个原本带有生存时间（TTL）的键来说， 当 SET 命令成功在这个键上执行时， 这个键原有的 TTL 将被清除。
    /// 返回值：
    ///     在 Redis 2.6.12 版本以前， SET 命令总是返回 OK 。
    ///     从 Redis 2.6.12 版本开始， SET 在设置操作成功完成时，才返回 OK 。
    /// PX millisecond ：设置键的过期时间为 millisecond 毫秒。
    /// # params : [`millisecond`] 单位为`毫秒`
    /// ```
    ///         let x = redis_rs::RedisClient::from("123456").connection().await.unwrap();
    ///         let state = x.set_string_px("gxk", "gxk", 50000).await.unwrap();
    ///         match state {
    ///             redis_rs::State::OK(v) => {
    ///                 println!("ok > {}", v)
    ///             }
    ///             redis_rs::State::ERROR(msg) => {
    ///                 println!("err > {}", msg)
    ///             }
    ///         }
    /// ```
    pub async fn set_string_px(&self,  k: &str, v: &str, millisecond: usize) -> io::Result<State<String>> {
        if v.len() <= 0 {
            panic!("value 不能为空!")
        }
        let vec = self.assembly_and_send(&["SET", k, v, "PX", &millisecond.to_string()]).await?;
        let msg = util::to_string(&vec[1..vec.len() - 2]);
        return if vec[0] as char == '-' {
            Ok(State::ERROR(msg))
        } else {
            Ok(State::OK(msg))
        }
    }

    /// 返回 key 所关联的字符串值。
    /// 如果 key 不存在那么返回 None 。
    /// 假如 key 储存的值不是字符串类型，返回一个错误，因为 GET 只能用于处理字符串值。
    /// ```
    ///         let x = redis_rs::RedisClient::from("123456").connection().await.unwrap();
    ///         let state = x.get_string("gxk").await.unwrap();
    ///         match state {
    ///             redis_rs::State::OK(v) => {
    ///                 match v {
    ///                     None => println!("值为空"),
    ///                     Some(s) => println!("查询成功 -> {}", s)
    ///                 }
    ///             }
    ///             redis_rs::State::ERROR(msg) => println!("err > {}", msg)
    ///         }
    /// ```
    pub async fn get_string(&self, k: &str) -> io::Result<State<Option<String>>> {
        let vec = self.assembly_and_send(&["GET", k]).await?;
        let msg = util::to_string(&vec[1..vec.len() - 2]);
        return if vec[0] as char == '-' {
            Ok(State::ERROR(msg))
        } else if msg == "-1" {
            Ok(State::OK(None))
        } else {
            let mut lines = vec.as_slice().lines();
            lines.next();
            Ok(match lines.next() {
                None => State::OK(None),
                Some(msg) => State::OK(Some(msg?)),
            })
        }
    }

    /// 如果 key 已经存在并且是一个字符串， APPEND 命令将 value 追加到 key 原来的值的末尾。
    /// 如果 key 不存在， APPEND 就简单地将给定 key 设为 value ，就像执行 SET key value 一样。
    /// ```
    ///         let x = redis_rs::RedisClient::from("123456").connection().await.unwrap();
    ///         let state = x.append("166", "123").await.unwrap();
    ///         match state {
    ///             redis_rs::State::OK(v) => {
    ///                 println!("ok > {}", v)
    ///             }
    ///             redis_rs::State::ERROR(msg) => {
    ///                 println!("err > {}", msg)
    ///             }
    ///         }
    /// ```
    pub async fn append(&self, k: &str, v: &str) -> Result<State<u32>, Box<dyn std::error::Error>> {
        if v.len() <= 0 {
            panic!("value 不能为空!")
        }
        let vec = self.assembly_and_send(&["APPEND", k, v]).await?;
        let string = util::to_string(&vec[1..vec.len() - 2]);
        return if vec[0] as char == '-' {
            Ok(State::ERROR(string))
        } else {
            Ok(State::OK(string.parse::<u32>()?))
        }
    }
}