use super::method::Method
use std::convert::TryFrom
use crate::http::Request

pub struct Request{
    path:String,
    query_string: Option<String>,
    method: Method,
}
impl Request{
    fn from_byte_array(but:&[u8]) -> Result<Self, String>{}
}

impl TryFrom<&[u8]> for Request {
    type Error = String;
    
    fn try_from(value:&[u8])-> Result<Self,Self::Error>{
        unimplemented!()
    }
}