use std::fs::File;
use std::net::TcpStream;
use std::io::{Read, Write, BufReader};
use regex::Regex;
use crate::core::*;

pub fn send(stream: &mut TcpStream, buf: &[u8]) -> Result<(), String> {
    match stream.write(buf) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string()),
    };

    match stream.flush() {
        Ok(_) => Ok(()),
        Err(e) => return Err(e.to_string()),
    }
}

pub fn recv_msg(stream: &mut TcpStream) -> Result<String, String> {
    let mut buf = [0; 512];
    let mut msg: Vec<u8> = Vec::new();
    
    loop {
        let size = match stream.read(&mut buf) {
            Ok(n) => n,
            Err(e) => return Err(e.to_string()),
        };

        for i in 0..size {
            msg.push(buf[i]);
        }

        if msg[msg.len()-1] == 0x0a && msg[msg.len()-2] == 0x0d {
            msg.pop();
            msg.pop();
            break;
        }
    }

    Ok(String::from_utf8_lossy(&msg).to_string())
}

pub fn recv_data(stream: &mut TcpStream) -> Result<Vec<u8>, String> {
    let mut data: Vec<u8> = Vec::new();

    match stream.read_to_end(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => return Err(e.to_string()),
    }
}

pub fn open_file(path: &str) -> Result<Vec<u8>, String> {
    let mut data: Vec<u8> = Vec::new();
    
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    match BufReader::new(file).read_to_end(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => return Err(e.to_string()),
    }
}

pub fn write_file(path: &str, data: &Vec<u8>) -> Result<(), String> {
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    match file.write_all(data) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e.to_string()),
    }
}

pub fn pasv(stream: &mut TcpStream) -> Result<TcpStream, String> {
    match send(stream, "PASV\r\n".as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };  

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "227" {
        return Err(msg);
    }

    let re = match Regex::new(r"(\d{1,3}),(\d{1,3}),(\d{1,3}),(\d{1,3}),(\d{1,5}),(\d{1,5})") {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    let ip = match re.captures(&msg) {
        Some(i) => i,
        None    => return Err("IP parsing error".to_string()),
    };

    match TcpStream::connect(format!("{}.{}.{}.{}:{}", &ip[1], &ip[2], &ip[3], &ip[4], &ip[5].parse::<i32>().unwrap()*256 + ip[6].parse::<i32>().unwrap())) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}