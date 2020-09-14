use super::buftcpstream::*;
use super::config::*;
use std::fmt::Write;
use std::io;

extern crate reqwest;

extern crate json;

pub struct Quiz<'a> {
    pub nickname: String,
    ss: &'a mut BufTcpStream,
    player: json::JsonValue,
    problems: json::JsonValue,
}
impl<'a> Quiz<'a> {
    pub fn new(ss: &'a mut BufTcpStream) -> io::Result<Self> {
        ss.print("Please Play This Game With UTF-8 Encode\n");
        ss.print("Hello World\n我們是電腦與網路愛好社 CCNS 感謝您遊玩本遊戲\n期待能在社博以及社大與您相見\n");
        ss.print("社大時間為 9/24 晚上 歡迎你\n");
        ss.print("Please enter your nickname:\n");
        let nickname = ss.read();

        let mut name = String::from("");
        write!(name, "{}{}", "nc-", nickname).unwrap();
        let server = String::from(QUIZ_SERVER);

        let data = json::object! {
            name: name.as_str(),
            nickname: nickname.as_str(),
            platform: "nc",
        };
        let player = json_post(server.clone() + "/v1/players", data);

        if player["status"]["status_code"] != 201 {
            println!("{}", player["status"]["status_code"]);
            ss.print("Nickname duplicate, please reconnect and input another.\n");
            panic!()
        }

        let problems = get(server.clone() + "/v1/quizzes");

        if problems["status"]["status_code"] != 200 {
            ss.print("!!!Quiz Server Failed!!!");
            println!("[Error] Get problems failed");
            panic!()
        }

        Ok(Self {
            nickname,
            ss,
            player,
            problems,
        })
    }
    pub fn get_problems_count(&self) -> i32 {
        self.problems["data"].len() as i32
    }
    pub fn print_problem(&mut self, i: i32) {
        self.ss.print(
            self.problems["data"][i as usize]["description"]
                .as_str()
                .unwrap(),
        );
        self.ss.print("\n");
        self.ss.print(
            self.problems["data"][i as usize]["option_a"]
                .as_str()
                .unwrap(),
        );
        self.ss.print("\n");
        self.ss.print(
            self.problems["data"][i as usize]["option_b"]
                .as_str()
                .unwrap(),
        );
        self.ss.print("\n");
        self.ss.print(
            self.problems["data"][i as usize]["option_c"]
                .as_str()
                .unwrap(),
        );
        self.ss.print("\n");
        self.ss.print(
            self.problems["data"][i as usize]["option_d"]
                .as_str()
                .unwrap(),
        );
        self.ss.print("\n\nhint:\n");
        self.ss
            .print(self.problems["data"][i as usize]["hint"].as_str().unwrap());
        self.ss.print("\n\nScore: ");

        self.ss.print(
            self.problems["data"][i as usize]["score"]
                .as_i32()
                .unwrap()
                .to_string(),
        );
        self.ss.print(" Point(s).\n");
    }
    pub fn ans_problem(&mut self, i: i32) {
        let mut input: String;
        loop {
            self.ss.print("Please input answer[A,B,C,D]\n");
            input = self.ss.read();
            match input.as_str() {
                "A" | "B" | "C" | "D" => break,
                _ => self
                    .ss
                    .print("Input syntax wrong, please answer again[A,B,C,D]\n"),
            }
        }
        let mut correctness = false;
        if input == self.problems["data"][i as usize]["answer"].to_string() {
            self.ss.print("----------\nCorrect!\n----------\n");
            correctness = true;
        } else {
            self.ss.print("----------\nFalse!\n----------\n");
        }
        let data = json::object! {
            player_name: self.player["data"]["name"].as_str(),
            quiz_number: self.problems["data"][i as usize]["number"].as_i32(),
            correct: correctness,
        };
        let server = String::from(QUIZ_SERVER);
        let response = json_post(server + "/v1/answers", data);
        if response["status"]["status_code"] != 201 {
            println!(
                "[Error] 送答案失敗 {} 不知道為什麼",
                response["status"]["status_code"]
            );
            self.ss.print("!!!Sorry Request Failed!!!\n");
            panic!()
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.ss.print("AI 出題中...\n\n");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    pub fn end_quiz(&mut self) {
        self.ss.print(".");
        std::thread::sleep(std::time::Duration::from_secs_f32(0.3));
        self.ss.print(".");
        std::thread::sleep(std::time::Duration::from_secs_f32(0.3));
        self.ss.print(".");
        std::thread::sleep(std::time::Duration::from_secs_f32(0.3));
        self.ss.print(".");
        std::thread::sleep(std::time::Duration::from_secs_f32(0.3));
        self.ss.print(".\n");
        std::thread::sleep(std::time::Duration::from_secs_f32(0.3));
        self.ss.print("AI 故障 沒有題目了\n");
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.ss.print("您的得分: ");

        let server = String::from(QUIZ_SERVER) + "/v1/players/";
        let response = get(server + self.player["data"]["name"].as_str().unwrap());

        self.ss
            .print(response["data"]["score"].as_i32().unwrap().to_string());
        self.ss.print("\n計分板: https://leaderboard.ccns.io\n");
        self.ss.print("\n社大時間為 9/24 晚上 歡迎你\n");
    }

    pub fn echo(&mut self) {
        let message = self.ss.read();
        self.ss.print(message);
    }
}
fn json_post(url: String, data: json::JsonValue) -> json::JsonValue {
    println!("\n\n{}", json::stringify(data.clone()));
    let client = reqwest::blocking::Client::new();
    let response = json::parse(
        client
            .post(url.as_str())
            .body(json::stringify(data))
            .send()
            .unwrap()
            .text()
            .unwrap()
            .as_str(),
    )
    .unwrap();
    println!("{}", response);
    response
}
fn get(url: String) -> json::JsonValue {
    let client = reqwest::blocking::Client::new();
    let response = json::parse(
        client
            .get(url.clone().as_str())
            .send()
            .unwrap()
            .text()
            .unwrap()
            .as_str(),
    )
    .unwrap();

    response
}
