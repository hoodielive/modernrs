use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub struct Request
{
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request 
{
    fn from_byte_array(buf: &[u8]) -> Result<Self, String>
    {
    }
}

impl TryFrom<&[u8]> for Request
{
   type Error = String; 

   fn try_from(value: &[u8]) -> Result<Self, Self::Error> 
   {
       let string = String::from("asd");
    //    string.encrypt();
    //   buf.encrypt();
       unimplemented!() 
   }
}

trait Encrypt
{
   fn encrypt(&self) -> Self; 
}

impl Encrypt for String
{
    fn encrypt(&self) -> Self 
    {
        unimplemented!();
    }
}

impl Encrypt for &[u8]
{
   fn encrypt(&self)  -> Self
   {
       unimplemented!()
   }
    
}

pub enum ParseError
{
    InvalidRequest, 
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParseError
{
   fn fmt(&self, f: &mut Formatter) -> FmtResult
   {
        write!(f, "{}", self.message())
   }
}

impl ParseError
{
   fn message(&self)->&str
   {
       match self
       {
            Self::InvalidRequest => "Invalid Request", 
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
       }
   }
}
