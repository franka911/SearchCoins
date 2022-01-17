# SearchCoins

Rust project. 
It downloads from CoinMarketCal website coin events, which are not enlisted to be avoided,
for given by the user period and sends email to user. 
In order program to run properly it must be supplied with input file, containing input data such as:
- smtp address of email with which data will be sent to target email and also 
username and password to it, 
- address of target email
- start and end of range of time to search 
- words that should be excluded from search

To check how the file should be structured please see test file in directory test_files.
Module ReadInitialData is responsible for correct reading of input data. 
Module HtmlScrapper is responsible for connecting to website CoinMarketCal, converting and checking 
correctness of given time range downloading HTML contents, coins searching.
Module EmailSender sends found coins to target email.
Module LibErrors has custom defined errors.