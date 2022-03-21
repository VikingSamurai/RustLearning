use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug,Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
use super::{QueryString,QueryStringValue};

#[derive(Debug)]
pub struct Request<'a>{
    path: &'a str,
    query_string: Option<QueryString<'a>>,
    method: Method,
}
//impl Request{
//    fn from_byte_array(but:&[u8]) -> Result<Self, String>{}
//}

impl<'a> Request<'a>{
    pub fn path(&self) -> &str{
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_string(&self)-> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = ParseError;
    
    fn try_from(buf:&[u8])-> Result<Self,Self::Error>{
        
        let request = str::from_utf8(buf)?;

        let (method, request)= get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request)= get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _)= get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol!="HTTP/1.1"{
            return Err(ParseError::InvalidProtocol)
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        
        if let Some(i) = path.find('?'){
            query_string = Some(&path[i+1..]);
            path = &path[..i];
        }
        
        unimplemented!()
    }
}

fn get_next_word(request: &str)-> Option<(&str,&str)> {
    request.chars();
    
    for (i,c) in request.chars().enumerate(){
        if c==' ' || c=='\r'{
            return Some((&request[..i],&request[i+1..]))
        }
    
    }
    unimplemented!()
}

//ParseError is a custom error type
pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError{
    fn message(&self)->&str{
        match self{
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self{
        Self::InvalidEncoding
    }
}


impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult
    {
        write!(f,"{}",self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult
    {
        write!(f,"{}",self.message())
    }
}

impl Error for ParseError{

}