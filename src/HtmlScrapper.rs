#![allow(warnings)]
extern crate reqwest;
extern crate tokio;
extern crate scraper;
extern crate select;
extern crate chrono;




use scraper::{Html, Selector};
use futures::executor::block_on;

use std::fs::File;
use std::path::Path;
use std::io::{Write, Read};
use std::collections::HashMap;
use std::ops::Index;
use select::document::Document;
use select::predicate::{Class, Name, Predicate, Attr};
use futures::{TryFutureExt, StreamExt};
use chrono::prelude::*;
use liberrors::myErrors;

#[derive(Debug, Copy, Clone)]
enum Dates{
    Jan = 01,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}


pub struct HTMLRequest {
    pub mySearch: Vec<Vec<String>>
}

impl HTMLRequest {
    pub fn new() -> Self {
        HTMLRequest { mySearch: Vec::new() }
    }


    fn dateToValue(&self, token_date: &str) -> Result< String, myErrors>{

        let mut split_date = token_date.split(" ").collect::<Vec<&str>>();

        let number  = match split_date[1]{
            "Jan" => (Dates::Jan as u8).to_string(),
            "Feb" => (Dates::Feb as u8).to_string(),
            "Mar"=> (Dates::Mar as u8).to_string(),
            "Apr"=> (Dates::Apr as u8).to_string(),
            "May"=> (Dates::May as u8).to_string(),
            "Jun"=> (Dates::Jun as u8).to_string(),
            "Jul"=> (Dates::Jul as u8).to_string(),
            "Sep"=> (Dates::Sep as u8).to_string(),
            "Oct"=> (Dates::Oct as u8).to_string(),
            "Nov"=> (Dates::Nov as u8).to_string(),
            "Dec"=> (Dates::Dec as u8).to_string(),
            _ => "".to_string(),
        };

        split_date[1] = &number;
        let converted_date = split_date[0].to_owned() + "-" + &split_date[1] + "-"+ &split_date[2];
        Ok(converted_date)

    }


    fn filter_data(&mut self, not_data: &[&str], search_results: &mut[Vec<String>]){

        for searches in search_results.iter_mut() {
            for dates in not_data.iter() {
                if searches[1].contains(&dates.to_lowercase()) {
                      searches.retain(|x| *x == *dates);

                    break;
                }
            }
            if (!searches.is_empty()) {
                self.mySearch.push(searches.clone());
            }
        }
    }

    pub async fn fetchSite(&mut self, url: &str, first_date: &str, second_date: &str, not_data:&[&str]) -> Result<(), myErrors> {


        let mut converted_date = self.convertDate(first_date, second_date)?;
        let mut page_number = 1i32;
        let mut url_address = url.to_string() + &page_number.to_string();

        let client = reqwest::Client::builder().build()?;

        let mut search_result: Vec<Vec<String>> = Vec::new();

        let permanent_converted_date = converted_date[0];
        while (converted_date[0] <= converted_date[1]) {

            let res = client.get(&url_address).send().await?.bytes().await?;

            let document = Document::from_read(res.as_ref())?;

            let mut card_counter= 16u8;

            for node in document.find(Name("article")) {
                let token_date = node.find(Class("card__date")).next().unwrap().children().next().unwrap().text();
                let dateValue_token_date = self.dateToValue(&token_date)?;

                let converted_token_date = NaiveDate::parse_from_str(&dateValue_token_date, "%d-%m-%Y")?;
                card_counter -= 1u8;



                if (permanent_converted_date <= converted_token_date && converted_token_date <= converted_date[1]){
                    let token_name = node.find(Class("card__coins")).next().unwrap().descendants().next().unwrap().next().unwrap().children().next().unwrap().text();
                    let token_date = node.find(Class("card__date")).next().unwrap().children().next().unwrap().text();
                    let mut token_event = node.find(Class("card__title")).next().unwrap().children().next().unwrap().text().to_lowercase();

                    let temp_vector = vec![token_name, token_event.trim().to_string(), token_date];

                    search_result.push(temp_vector);

                }

                converted_date[0] = converted_token_date;

            }

            page_number += 1i32;
            url_address = url.to_string() + &page_number.to_string();

        }

        self.filter_data(not_data, &mut search_result);
        Ok(())
    }

    fn convertDate(&self, first_date: &str, second_date: &str) -> Result<Vec<NaiveDate>, myErrors> {
        let today_date = Utc::now().date().naive_utc();
        let last_avaiable_date = NaiveDate::from_ymd(2024, 12, 31);
        let converted_first_date = NaiveDate::parse_from_str(first_date, "%d-%m-%Y")?;
        let converted_second_date = NaiveDate::parse_from_str(second_date, "%d-%m-%Y")?;

        if converted_first_date <= converted_second_date {
            if converted_first_date < today_date {
                panic!("First date Less then today date");
            }
            if converted_first_date > last_avaiable_date {
                panic!("First date more then last avaiable date");
            }

            if converted_second_date < today_date {
                panic!("Second date Less then today date");
            }
            if converted_second_date > last_avaiable_date {
                panic!("Second date more then last avaiable date");
            }
        } else {
            panic!("First given date more then last given date");
        }

        Ok(vec![converted_first_date,converted_second_date])
    }
}


#[cfg(test)]
mod tests {
    use super::*;


//test in order to find if date is coverted right
    //tested function convertDate

    #[test]
    fn UnitTest_CatchDateParse() {
        let first_date = "abc";
        let second_date = "07-12-2021";
        let mySearchCoins = HTMLRequest::new();
        let result = mySearchCoins.convertDate(&first_date, &second_date);
        assert!(result.is_err())

    }

    #[test]
    #[should_panic]
    fn UnitTest_SecondDateLessThanToday() {
        let first_date = "07-12-2021";
        let second_date = "04-12-2021";
        let mySearchCoins = HTMLRequest::new();
        mySearchCoins.convertDate(&first_date, &second_date);

    }

    #[test]
    #[should_panic]
    fn UnitTest_SecondDateMoreThanAvaiable() {
        let first_date = "07-12-2021";
        let second_date = "04-12-2040";
        let mySearchCoins = HTMLRequest::new();
        mySearchCoins.convertDate(&first_date, &second_date);

    }

    #[tokio::test]
    async fn UnitTest_WrongURL() {
        let url=  "https://coinmarketca3l.com/en/?page=";
        let first_date = "01-02-2022";
        let second_date = "02-02-2022";
        let not_data = vec!["abc"];
        let mut mySearchCoins = HTMLRequest::new();
        let result = mySearchCoins.fetchSite(&url, &first_date, &second_date,&not_data).await;
        assert!(result.is_err())

    }

    #[test]
    fn UnitTest_GoodVectorFiltering() {

        let token_date = "07 Nov 2021";
        let mySearchCoins = HTMLRequest::new();
        let result = mySearchCoins.dateToValue(token_date).unwrap();
        assert_eq!(result, "07-11-2021");

    }

}
