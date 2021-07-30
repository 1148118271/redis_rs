use std::net::TcpStream;

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




    #[test]
    fn test1() {
        let a = "aa\r\nbb\r\n".as_bytes();
        let mut result = a.lines();
       // let mut split = a.split(|v| *v as char == '\r' || *v as char == '\n');
        println!("{:?}", result.next().unwrap());
        println!("{:?}", result.next().unwrap());

    }
}