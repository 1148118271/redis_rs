use crate::redis_instance::{RedisInstance, Validation};
use std::io;
use crate::result;
use crate::util;
use crate::result::State;
use std::io::BufRead;

struct RString;

impl Validation<String> for RString {

    fn validation(v: &[u8]) -> result::Result<String> {
        let r = util::to_string(&v[1..v.len() - 2]);
        return if "-1".eq(&r) {
            result::Result::new(State::NULL)
        } else {
            if v[0] as char == '-' {
                result::Result::new(State::ERROR)
            } else {
                result::Result::new(State::OK)
            }
        }

    }
}

impl RedisInstance {
    pub async fn set_string(&self, k: &str, v: &str)
        -> io::Result<result::Result<String>>
    {
        let vec = self.assembly_and_send(&["SET", k, v]).await?;
        let mut result = RString::validation(&vec);
        match result.state {
            State::OK => {
                result.from(util::to_string(&vec[1..vec.len() - 2]))
            }
            State::ERROR => {
                result.from(util::to_string(&vec[1..vec.len() - 2]))
            }
            _ => {}
        }
        Ok(result)
    }

    pub async fn get_string(&self, k: &str) -> io::Result<result::Result<String>> {
        let vec = self.assembly_and_send(&["GET", k]).await?;
        let mut result = RString::validation(&vec);
        match result.state {
            State::OK => {
                let mut lines = vec.as_slice().lines();
                lines.next();
                let x = lines.next().unwrap().unwrap();
                result.from(util::to_string(x.as_bytes()))
            }
            State::ERROR => {
                result.from(util::to_string(&vec[1..vec.len() - 2]))
            }
            _ => {}
        }
        Ok(result)
    }
}