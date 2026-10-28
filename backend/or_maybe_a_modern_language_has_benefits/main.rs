use std::io::{BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::env;

fn print_inc(sock: TcpStream) {
    let mut sock_reader = BufReader::new(sock);

    loop {
        let mut buffer : Vec<u8> = vec![];
        let n = sock_reader.read_until(b'\0', &mut buffer);
        if n.unwrap() > 0 {
            let msg = String::from_utf8(buffer).unwrap();
            println!("{}", msg);
        }
    }
}

fn main() {
    println!("hello world");
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        print_inc(stream.unwrap());
    }
}
