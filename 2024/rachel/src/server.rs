use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::Try;
use std::io::Read;

pub struct Server
{
   addr: String, 
}

impl Server
{
    // new is an associated function called directly on the struct, not an instance.
    
    pub fn new(addr: String) -> Self // alias for Struct itself
    {
        Server 
        {
            addr,
        }
    }

    pub fn run(self)
    {
        println!("Listening on {}", self.addr);

        
        let _listener = TcpListener::bind(&self.addr).unwrap(); // return tcpListener or Error
      
        // Accept returns (TCPStream, SocketAddr)

        loop 
        { 
            let _res = _listener.accept(); 

            match _res
            {
                Ok((mut stream, _)) => 
                {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer)
                    {
                        Ok(_) => 
                        { 
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            
                            //Request::try_from(&buffer as &[u8]);
                            
                            match Request::try_from(&buffer[..]) 
                            {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e),
                            };
                            
                            let resu: &Result<Request, _> = &buffer[..].try_into();
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
