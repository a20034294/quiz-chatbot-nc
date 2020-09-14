use std::net::{TcpListener, TcpStream};
use std::thread;

pub mod quiz;
use quiz::*;
pub mod buftcpstream;
use buftcpstream::*;
pub mod config;

fn handle_client(stream: TcpStream) {
    let (ip, port) = (
        stream.peer_addr().unwrap().ip(),
        stream.peer_addr().unwrap().port(),
    );
    let mut ss = BufTcpStream::new(stream).unwrap();

    println!("Client Connect From: {}:{}", ip, port);
    let mut quiz = Quiz::new(&mut ss).unwrap();
    for p_now in 0..quiz.get_problems_count() {
        quiz.print_problem(p_now);
        quiz.ans_problem(p_now);
    }
    quiz.end_quiz();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8899")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || handle_client(stream));
    }
    Ok(())
}
