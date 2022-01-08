#![allow(warnings)]
use std::fmt;

#[derive(Debug)]
pub enum myErrors{
    UrlError,
    IoError,
    TimeParseError,
}

impl std::error::Error for myErrors{}


impl fmt::Display for myErrors{
    fn fmt (&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            myErrors::UrlError => write!(f, "HTTP URL parse error"),
            myErrors::IoError=> write!(f, "IO Parse error" ),
            myErrors::TimeParseError => write!(f, "Given time not parsed "),

        }
    }
}


impl From<std::io::Error> for myErrors{
    fn from(err: std::io::Error) -> Self{
        myErrors::IoError
    }
}

impl From<reqwest::Error> for myErrors{
    fn from(err: reqwest::Error) -> Self{
        myErrors::UrlError
    }
}

impl From<chrono::format::ParseError> for myErrors{
    fn from(err: chrono::format::ParseError) -> Self{
        myErrors::TimeParseError
    }
}



