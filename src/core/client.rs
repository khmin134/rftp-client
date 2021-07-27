use crate::core::*;

pub fn client_loop() {
    let mut stream = match cmd::open() {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match cmd::login(&mut stream) {
        Ok(m) => println!("{}", m),
        Err(e) => {
            println!("{}", e);
            return;
        },
    }

    match cmd::bin_type(&mut stream) {
        Ok(m) => println!("{}", m),
        Err(e) => println!("{}", e),
    }

    loop {
        let cmd = util::print_and_read("rftp> ");
        let cmds = util::msg_spliter(&cmd);

        match cmds[0] {
            "help" => println!("{}", cmd::help()),
            "pwd" => util::print_and_chkerr(cmd::pwd(&mut stream)),
            "ls" => util::print_and_chkerr(cmd::list(&mut stream)),
            "cd" => util::print_and_chkerr(cmd::cd(&mut stream, cmds)),
            "up" => util::print_and_chkerr(cmd::stor(&mut stream, cmds)),
            "down" => util::print_and_chkerr(cmd::retr(&mut stream, cmds)),
            "mkdir" =>util::print_and_chkerr(cmd::mkd(&mut stream, cmds)),
            "rm" => util::print_and_chkerr(cmd::dele(&mut stream, cmds)),
            "rmd" => util::print_and_chkerr(cmd::rmd(&mut stream, cmds)),
            "mv" => util::print_and_chkerr(cmd::rnto(&mut stream, cmds)),
            "size" => util::print_and_chkerr(cmd::size(&mut stream, cmds)),
            "bye" => return,
            _ => println!("Unkown command!"),
        }
    }
}