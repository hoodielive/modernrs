use http::Request;
use http::Method;
use server::Server;

mod http;
mod server;

fn main() 
{
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}

