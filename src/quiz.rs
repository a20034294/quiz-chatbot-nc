use super::buftcpstream::*;
use super::config::*;
use std::fmt::Write;
use std::io;

extern crate reqwest;
use std::collections::HashMap;

extern crate json;

pub struct Quiz<'a> {
    pub nickname: String,
    pub p_now: i32,
    ss: &'a mut BufTcpStream,
    //quizs
}
impl<'a> Quiz<'a> {
    pub fn new(ss: &'a mut BufTcpStream) -> io::Result<Self> {
        ss.print(String::from("Please enter your nickname:\n"));
        let nickname = ss.read();
        let p_now = 1;

        let mut name = String::from("");
        write!(name, "{}{}", "nc-", nickname).unwrap();
        let mut data = HashMap::new();
        data.insert("name", name.as_str());
        data.insert("nickname", nickname.as_str());
        data.insert("platform", "nc");

        let mut url = String::from("");
        write!(&mut url, "{}{}", QUIZ_SERVER, "/v1/players").unwrap();
        let client = reqwest::blocking::Client::new();
        let response = json::parse(
            client
                .post(url.as_str())
                .json(&data)
                .send()
                .unwrap()
                .text()
                .unwrap()
                .as_str(),
        )
        .unwrap();

        println!("{}", response["status"]["status_code"]);

        Ok(Self {
            nickname,
            p_now,
            ss,
        })
    }
    pub fn echo(&mut self) {
        let message = self.ss.read();
        self.ss.print(message);
    }
}
