use std::net::{TcpStream, ToSocketAddrs, Shutdown};
use std::time::SystemTime;
use crate::core::*;

pub fn help() -> String {
    "
pwd   : Print working directory
ls    : Print files located in the working directory
cd    : Change working directory
up    : Upload file to server
down  : Download file from server
mkdir : Make directory
rm    : Remove a file
rmd   : Remove a direcotry
mv    : Change the name or path of a file or directory
size  : Get the file size
    ".to_string()
}

pub fn open() -> Result<TcpStream, String> {
    let mut hostname = util::print_and_read("Hostname : ");

    if hostname.find(':') == None {
        hostname += ":21";
    }

    let mut addr = match hostname.to_socket_addrs() {
        Ok(a) => a,
        Err(e) => return Err(e.to_string()),
    };

    let addr = match addr.next() {
        Some(a) => a,
        None => return Err("Incorrect hostname".to_string()),
    };

    let mut stream = match TcpStream::connect(addr) {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };

    let msg = match ftpio::recv_msg(&mut stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "220" {
        return Err("The server is not ready!".to_string());
    }
    
    println!("{}", msg);
    Ok(stream)    
}

pub fn login(stream: &mut TcpStream) -> Result<String, String> {
    println!("If you want anonymous access, enter a space");
    let id = util::print_and_read("Username : ");

    match ftpio::send(stream, format!("USER {}\r\n", id).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "331" {
        return Err(msg);
    }

    let pw = util::print_and_read("Password : ");

    match ftpio::send(stream, format!("PASS {}\r\n", pw).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "230" {
        return Err(msg);
    }

    Ok(msg)
}

pub fn bin_type(stream: &mut TcpStream) -> Result<String, String> {
    match ftpio::send(stream, "TYPE I\r\n".as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);
    
    if msgs[0] != "200" {
        return Err(msg);
    }

    Ok(msg)
}

pub fn pwd(stream: &mut TcpStream) -> Result<String, String> {
    match ftpio::send(stream, "PWD\r\n".as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "257" {
        return Err(msg);
    }

    Ok(msg)
}

pub fn list(stream: &mut TcpStream) -> Result<String, String> {
    let mut pasvStream = match ftpio::pasv(stream) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match ftpio::send(stream, "LIST\r\n".as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "150" {
        return Err(msg);
    }

    let data = match ftpio::recv_data(&mut pasvStream) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    
    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "226" {
        return Err(msg);
    }

    Ok(String::from_utf8_lossy(&data).trim().to_string())
}

pub fn cd(stream: &mut TcpStream, args: Vec<&str>) -> Result<String, String> {
    let path: String;

    if args.len() == 2 {
        path = args[1].to_string();
    } else {
        path = util::print_and_read("Path : ");
    }
    
    match ftpio::send(stream, format!("CWD {}\r\n", path).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "250" {
        return Err(msg);
    }

    Ok(msg)
}

pub fn stor(stream: &mut TcpStream, args: Vec<&str>) -> Result<String, String> {
    let remoteFile: String;
    let localFile: String;

    if args.len() == 3 {
        localFile = args[1].to_string();
        remoteFile = args[2].to_string();
    } else {
        localFile = util::print_and_read("Local file : ");
        remoteFile = util::print_and_read("Remote file : ");
    }

    let data = match ftpio::open_file(&localFile) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };

    let mut pasvStream = match ftpio::pasv(stream) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match ftpio::send(stream, format!("STOR {}\r\n", remoteFile).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "150" {
        return Err(msg);
    }

    let now = SystemTime::now();

    match ftpio::send(&mut pasvStream, &data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match pasvStream.shutdown(Shutdown::Both) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string()),
    }

    let elapsed = match now.elapsed() {
        Ok(t) => format!("{}sec", t.as_secs_f64()),
        Err(e) => e.to_string(),
    };

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "226" {
        return Err(msg);
    }

    Ok(elapsed)
}

pub fn retr(stream: &mut TcpStream, args: Vec<&str>) -> Result<String, String> {
    let remoteFile: String;
    let localFile: String;

    if args.len() == 3 {
        remoteFile = args[1].to_string();
        localFile = args[2].to_string();
    } else {
        remoteFile = util::print_and_read("Remote file : ");
        localFile = util::print_and_read("Local file : ");
    }

    let mut pasvStream = match ftpio::pasv(stream) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match ftpio::send(stream, format!("RETR {}\r\n", remoteFile).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "150" {
        return Err(msg);
    }

    let now = SystemTime::now();

    let data = match ftpio::recv_data(&mut pasvStream) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };

    let elapsed = match now.elapsed() {
        Ok(t) => format!("{}sec", t.as_secs_f64()),
        Err(e) => e.to_string(),
    };

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "226" {
        return Err(msg);
    }

    match ftpio::write_file(&localFile, &data) {
        Ok(_) => Ok(elapsed),
        Err(e) => Err(e),
    }
}

pub fn mkd(stream: &mut TcpStream, args: Vec<&str>) -> Result<String, String> {
    let dir: String;

    if args.len() == 2 {
        dir = args[1].to_string();
    } else {
        dir  = util::print_and_read("New directory : ");
    }

    match ftpio::send(stream, format!("MKD {}\r\n", dir).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "257" {
        return Err(msg);
    }

    Ok(msg)
}

pub fn dele(stream: &mut TcpStream, args: Vec<&str>) -> Result<String, String> {
    let file: String;

    if args.len() == 2 {
        file = args[1].to_string();
    } else {
        file = util::print_and_read("File to delete : ");
    }

    match ftpio::send(stream, format!("DELE {}\r\n", file).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "250" {
        return Err(msg);
    }

    Ok(msg)
}

pub fn rmd(stream: &mut TcpStream, args: Vec<&str>) -> Result<String, String> {
    let dir: String;

    if args.len() == 2 {
        dir = args[1].to_string();
    } else {
        dir = util::print_and_read("Directory to delete : ");
    }

    match ftpio::send(stream, format!("RMD {}\r\n", dir).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "250" {
        return Err(msg);
    }

    Ok(msg)
}

pub fn rnto(stream: &mut TcpStream, args: Vec<&str>) -> Result<String, String> {
    let oriName: String;
    let newName: String;

    if args.len() == 3 {
        oriName = args[1].to_string();
        newName = args[2].to_string();
    } else {
        oriName = util::print_and_read("Original name : ");
        newName = util::print_and_read("New name : ");
    }

    match ftpio::send(stream, format!("RNFR {}\r\n", oriName).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "350" {
        return Err(msg);
    }

    match ftpio::send(stream, format!("RNTO {}\r\n", newName).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "250" {
        return Err(msg);
    }

    Ok(msg)
}

pub fn size(stream: &mut TcpStream, args: Vec<&str>) -> Result<String, String> {
    let file: String;

    if args.len() == 2 {
        file = args[1].to_string();
    } else {
        file = util::print_and_read("File : ");
    }

    match ftpio::send(stream, format!("SIZE {}\r\n", file).as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let msg = match ftpio::recv_msg(stream) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let msgs = util::msg_spliter(&msg);

    if msgs[0] != "213" {
        return Err(msg);
    }

    Ok(format!("{}byte", msgs[1]))
}