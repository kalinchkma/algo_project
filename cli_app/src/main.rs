use std::{env, process};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::io::{self, Write};


const MAX: u16 = u16::MAX;

struct Argument {
    threads: u16,
    ipaddr: IpAddr,
}

impl Argument {
    fn new(args: &[String]) -> Result<Argument, &str> {

        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }   
        
        let flag = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&flag) {

            return Ok(Argument {threads: 4, ipaddr});

        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {

                println!("Usage: -j to select how many threads you want
                \r\n      -h or --help to show this help message");

                return Err("help");

            } else if flag.contains("-h") || flag.contains("--help") {

                return Err("too many arguments");

            } else if flag.contains("-j") {

                let ipaddr = match IpAddr::from_str(&args[3]) {

                    Ok(f) => f,

                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {

                    Ok(f) => f,

                    Err(_) => return Err("failed to parse thread number"),
                };

                return Ok(Argument { threads, ipaddr});

            } else {

                return Err("invalid syntax");
            }
             
        }
    }
}


fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            },
            Err(_) => {}
        }

        if MAX - port < num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
   let args: Vec<String> = env::args().collect();
   let program = &args[0];

    println!("{:?}", args);
    
   let argument = Argument::new(&args).unwrap_or_else(|err| {

        if err.contains("help") {

            process::exit(0)
        } else {

            println!("{} problem parsing arguments: {}", program, err);

            process::exit(0)
        }
    });

    let num_threads = argument.threads;
    let (tx, rx) = channel();
    for i in 0 .. num_threads {
        let tx = tx.clone();
        
        thread::spawn(move || {
            scan(tx, i, argument.ipaddr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p)
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
