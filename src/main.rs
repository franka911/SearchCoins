#![allow(warnings)]
extern crate tokio;
use std::env;
use std::fs::{File, read};
use std::io::Read;

mod ReadInitialData;
mod HtmlScrapper;
mod EmailSender;

use ReadInitialData::read_File;
use HtmlScrapper::HTMLRequest;
use std::any::type_name;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {


    if env::args().len() < 2{
        panic!("Not enough arguments in command line");
    }
    let url = "https://coinmarketcal.com/en/?page=";
    let read_line = env::args().collect::<Vec<String>>();

    let mut f = File::open(&read_line[read_line.len()-1])?;

    let read_result = read_File(&mut f).expect("Data from file could not be read");

    let data_excluded_from_search = read_result.get("data_to_not_search").expect("did not find anything");

    let mut html_scrapper = HTMLRequest::new();

    let excluded_data=data_excluded_from_search.split(",").collect::<Vec<&str>>();

    html_scrapper.fetchSite( &url, &read_result.get("first_date").unwrap(), &read_result.get("second_date").unwrap(),
                             &excluded_data).await?;

    let mut search_result = String::new();

    //add space to vector, because vectors are written in one line in email
    search_result = html_scrapper.mySearch.into_iter().map(|x|
        format!("{:?} \n", x)).collect();


   EmailSender::send_email(&read_result.get("to_mail").unwrap(),&read_result.get("username").unwrap(),
                      &search_result, &read_result.get("smtp_address").unwrap(), &read_result.get("username").unwrap(),   &read_result.get("password").unwrap());
    Ok(())
}
