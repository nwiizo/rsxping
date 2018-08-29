use std::process::Command;

fn usage() {
        println!("rsxping <ipaddress>")
}

fn main() {
    let addr = match std::env::args().nth(1) {
        Some(addr) => addr,
        None => {
            usage();
            return;
        }
    };

    let ping =  Command::new("ping").arg("-c 1").arg(&addr).output().expect("failed to execute process");
    println!("{:?}",ping)
}
