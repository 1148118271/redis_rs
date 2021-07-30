use tokio::net::{ToSocketAddrs, TcpStream};
use tokio::io;

pub struct Client(TcpStream);


impl Client {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Client> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Client{ 0: stream })
    }

    pub async fn read(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        loop {
            self.0.readable().await?;

            match self.0.try_read(buf) {
                Ok(len) => {
                    buf.truncate(len);
                    return Ok(())
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => return Err(e.into())
            }
        }
    }

    pub async fn write(&self, buf: &[u8]) -> io::Result<()> {
        loop {
            self.0.writable().await?;

            match self.0.try_write(buf) {
                Ok(_) => return Ok(()),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => return Err(e.into())
            }
        }
    }

    pub async fn close(self) {
        drop(self);
    }

}
