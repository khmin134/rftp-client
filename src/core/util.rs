use std::io::{self, Write};

pub fn msg_spliter(msg: &str) -> Vec<&str> {
    let msgs: Vec<&str> = msg.trim().split(" ").collect();
    
    msgs
}

pub fn print_and_read(msg: &str) -> String {
    let mut r = String::new();

    print!("{}", msg);
    io::stdout().flush()
    .expect("There was a big problem with the I/O system\nExit the program");

    match io::stdin().read_line(&mut r) {
        Ok(_) => (),
        Err(_) => r.clear(),
    }

    r.trim().to_string()
}

static mut ERR_CNT: u32 = 0;

pub fn print_and_chkerr(r: Result<String, String>) {
    match r {
        Ok(m) => println!("{}", m),
        Err(e) => {
            unsafe {
                if ERR_CNT == 10 {
                    println!("
            Too many errors occurred
            Check your network and try to reconnect with the server
                    ");
                } else {
                    ERR_CNT += 1;
                }
            }

            println!("ERROR> {}", e);
        }
    }
}