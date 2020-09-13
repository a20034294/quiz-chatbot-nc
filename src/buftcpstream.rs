use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter};
use std::net::TcpStream;

pub struct BufTcpStream {
    pub input: BufReader<TcpStream>,
    pub output: BufWriter<TcpStream>,
}

impl BufTcpStream {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let input = BufReader::new(stream.try_clone()?);
        let output = BufWriter::new(stream);
        Ok(Self { input, output })
    }
    pub fn read(&mut self) -> String {
        let mut buffer = String::new();
        self.input.read_line(&mut buffer).unwrap();
        if buffer.len() == 0 {
            panic!()
        }
        buffer.pop();
        buffer
    }
}
pub trait Print<T> {
    fn print(&mut self, s: T);
}
impl Print<String> for BufTcpStream {
    fn print(&mut self, s: String) {
        write!(self.output, "{}", s).unwrap();
        self.output.flush().unwrap();
    }
}
impl Print<i32> for BufTcpStream {
    fn print(&mut self, s: i32) {
        write!(self.output, "{}", s).unwrap();
        self.output.flush().unwrap();
    }
}
impl Print<&'static str> for BufTcpStream {
    fn print(&mut self, s: &'static str) {
        write!(self.output, "{}", s).unwrap();
        self.output.flush().unwrap();
    }
}
