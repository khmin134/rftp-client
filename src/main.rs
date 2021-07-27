mod core;

use crate::core::{client, util};

fn main() {
    println!("--------------------------------------------------------------------------------");
    println!("|                              rftp-client v0.1.0                              |");
    println!("|                    The FTP client program written in Rust                    |");
    println!("|                                                                              |");
    println!("|If you need information about the program, read the README.md or visit github |");
    println!("|GitHub : https:://github.com/khmin134/rftp-client                             |");
    println!("--------------------------------------------------------------------------------");

    loop {
        client::client_loop();
        
        loop {
            let s = util::print_and_read("Do you want to reconnect? [y/n] ");

            if s == "y" {
                break;
            } else if s == "n" {
                return;
            } else {
                println!("Incorrect command");
            }
        }
    }
}
