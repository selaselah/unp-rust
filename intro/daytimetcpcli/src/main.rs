use std::env;
use std::net;
use std::io::Read;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("useage: daytimetcpcli <IPaddress>");
        panic!();
    }
    let host = args[1].trim();
    let port: u16 = 8013;

    let mut sock = net::TcpStream::connect((host, port))
        .ok().expect("create connect failed");

    let mut recvline = String::new();
    loop {
        match sock.read_to_string(&mut recvline) {
            Ok(0) => break,
            Ok(_) => println!("recv: {}", &recvline), 
            Err(e) => { println!("read error: {}", e); panic!(); }
        };
    }
    sock.shutdown(net::Shutdown::Both).unwrap();
}
