use crate::tcp::Client;
use tokio::io;
use std::rc::Rc;
use crate::result;

mod constant {
    pub const CRLF: &'static str = "\r\n";

    pub const LIST_SYMBOL: &'static str = "*";

    pub const STR_SYMBOL: &'static str = "$";
}


pub trait Validation<T> {
    fn validation(v: &[u8]) -> result::Result<T>;
}



pub struct RedisInstance(Client);

pub type Instance = RedisInstance;

impl RedisInstance {
    pub fn init(c: Client) -> Instance {
        RedisInstance(c)
    }

    pub async fn login(&self, pass: &str) -> io::Result<()> {
        self.assembly_and_send(&["AUTH", pass]).await?;
        Ok(())
    }

    pub async fn assembly_and_send(&self, data: &[&str]) -> io::Result<Vec<u8>> {
        let mut params = String::from(constant::LIST_SYMBOL);
        params += &data.len().to_string();
        params += constant::CRLF;
        for x in data {
            params += constant::STR_SYMBOL;
            params += &(*x.len().to_string());
            params += constant::CRLF;
            params += *x;
            params += constant::CRLF;
        }
        println!("Client -> {}", &params);
        self.0.write(params.as_bytes()).await?;
        let mut v = vec![0; 2048];
        self.0.read(&mut v).await?;
        let rc = Rc::new(v);
        let str = unsafe {
            String::from_utf8_unchecked(Rc::clone(&rc).to_vec())
        };
        println!("Server -> {}", str);
        Ok(Rc::clone(&rc).to_vec())
    }

    // // 43 / 45
    // pub async fn push_str(&self, k: &str, v: &str) -> io::Result<State> {
    //     let vec = self.assembly_and_send(&["SET", k, v]).await?;
    //     Ok(validation(&vec))
    // }
    //
    //
    // pub async fn push_list(&self, k: &str, v: &[&str]) -> io::Result<State> {
    //     let mut arr = vec![];
    //     arr.push("lpush");
    //     arr.push(k);
    //     arr.extend_from_slice(v);
    //     let vec = self.assembly_and_send(&arr).await?;
    //     Ok(validation(&vec))
    // }


}

