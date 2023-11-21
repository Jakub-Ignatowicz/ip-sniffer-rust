use std::env;
use std::net::IpAddr;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(afgs: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }else if args.len() > 4 {
            return Err("too many arguments");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for i in &args {
        println!("{}", i);
    }

    println!("{:?}", args);
}
