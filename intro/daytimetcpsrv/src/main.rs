extern crate time;
use std::net;
use std::io::Write;

fn main() {
    let port: u16 = 8013;
    let listensock = match net::TcpListener::bind(("localhost", port)) {
        Ok(sock) => sock,
        Err(e) => {
            println!("bind_error: {}", e); panic!();
        }
    };

    loop {
        let (mut conn, _) = listensock.accept().ok().expect("accept error");
        let tm = time::now();
        let sendline = time::strftime("%Y-%m-%d %H:%M:%S", &tm)
            .ok().expect("strftime error");
        println!("send: {}", sendline);
        conn.write(&sendline.as_bytes()).unwrap();
        println!("shutdown {}", conn.peer_addr().unwrap());
        conn.shutdown(net::Shutdown::Both).unwrap();
    }
}
