use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::Request;

pub struct Server{
    address: String,
}

impl Server{
    pub fn new(addr:String) -> Self{
        Self{
            address:addr
        }
    }
    pub fn run(self){
        println!("Listening on {}",self.address);

        let listener = TcpListener::bind(&self.address).unwrap();
        loop{
            match listener.accept(){
                Ok((mut stream, _)) => {
                    let mut buffer = [0;1024];
                    match stream.read(&mut buffer)
                    {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]){
                                Ok(request)=>{}
                                Err(e)=>println!("Failed to parse request {}",e)
                            }
                        // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connection: {}",e)
                    }  
                }
                Err(e) => println!("Failed to establish a connection: {}",e)
            }
        }
    }
}