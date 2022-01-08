#![allow(warnings)]
use std::env;
use std::fs::{File, read};
use std::io::Read;
use std::path::Path;
use futures::StreamExt;
use tokio::io::BufReader;
use std::collections::HashMap;



fn check_data (sdata: &str, read_data: &mut HashMap<String, String>) {

    let  splitted_data: _ = sdata.split(":").collect::<Vec<&str>>();
    match splitted_data[0]{
        "smtp_address" => read_data.insert(String::from(splitted_data[0]), String::from(splitted_data[1])),
        "username" => read_data.insert(String::from(splitted_data[0]), String::from(splitted_data[1])),
        "password" => read_data.insert(String::from(splitted_data[0]), String::from(splitted_data[1])),
        "first_date" => read_data.insert(String::from(splitted_data[0]), String::from(splitted_data[1])),
        "second_date" => read_data.insert(String::from(splitted_data[0]), splitted_data[1].parse().unwrap()),
        "data_to_not_search" => read_data.insert(String::from(splitted_data[0]), splitted_data[1].parse().unwrap()),
        "to_mail" => read_data.insert(String::from(splitted_data[0]), splitted_data[1].parse().unwrap()),
        _ => panic!("There is no such data avaiable. There should be values for: smtp_address,\
         username,password,first_date,second_date,data_to_not_search,to_mail"),
    };

}

pub fn read_File(f: &mut File) -> Option<HashMap<String,String>> {


    let mut read_data: HashMap<String, String> = HashMap::new();
    let mut new_buffer = String::new();
    let result = f.read_to_string(&mut new_buffer).expect("Could not read data from file correctly");

    if result == 0usize{
        return None;
    }

    for lines in new_buffer.lines(){

        let temp_line = lines.to_lowercase().trim().split_whitespace().collect::<String>();
        check_data(&temp_line, &mut read_data);
    }

    Some(read_data)

}


#[cfg(test)]
mod tests {
    use std::collections::btree_map::BTreeMap;
    use super::*;
    use std::iter::FromIterator;


    #[test]
    fn UnitTest_ReadFile_Correct_Values() {
        let mut path =  File::open("test_files/income_data_test_correct.txt").unwrap();
        let output = read_File(&mut path).unwrap();

        let correct_output: HashMap<String, String> = HashMap::from_iter(
            [("smtp_address".to_owned(), "smtp.gmail.com".to_owned()),
                ("username".to_owned(), "xyz".to_owned()),
                ("password".to_owned(), "xyz".to_owned()),
                ("first_date".to_owned(), "08-01-2022".to_owned()),
                ("second_date".to_owned(), "10-01-2022".to_owned()),
                ("data_to_not_search".to_owned(), "Listing, Drop, conv, ama".to_owned()),
                ("to_mail".to_owned(), "test.gmail.com".to_owned()),
            ]);
        let result = correct_output.len() == output.len() && output.keys().all(|k| correct_output.contains_key(k));
        assert!(result);
    }


    #[test]
    #[should_panic(expected = "There is no such data avaiable. There should be values for: smtp_address,\
         username,password,first_date,second_date,data_to_not_search,to_mail")]
    fn UnitTest_ReadFile_WrongValues() {
        let mut path =  File::open("test_files/income_data_test_wrong.txt").unwrap();
        let output = read_File(&mut path).unwrap();
    }

    #[test]
    fn UnitTest_ReadFile_File_Empty() {
        let mut path =  File::open("test_files/income_data_test_empty.txt").unwrap();
        let output = read_File(&mut path);
        assert!(output.is_none());
    }

}