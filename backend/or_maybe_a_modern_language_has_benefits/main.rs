// use std::env;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::array::from_fn;
use std::collections::VecDeque;


fn handle_tcp_conn(sock: TcpStream, inc_dump: Sender<Message>, response: Receiver<Message>) {
    let mut sock_reader = BufReader::new(sock);

    loop {
        let mut buffer: Vec<u8> = vec![];
        let n_chars = sock_reader.read_until(b'\0', &mut buffer);
        if n_chars.unwrap() > 0 {
            let msg = String::from_utf8(buffer).unwrap();
            println!("{}", msg);
        }
    }
}


fn parse_message(raw_message: String) -> Option<Message> {
    
    return None;    
}

fn str_to_mtype(type_str: String) -> Option<Mtype> {
    None
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Mtype {
    Data,
    Query,
    Command,
    Misc,
    Invalid
}

#[derive(Clone)]
struct Message {
    pub id: usize,
    pub from: String,
    pub mtype: Mtype,
    pub content: String
}

impl Message {
    pub fn new_empty() -> Self {
        Self {
            id: 0,
            from: String::new(),
            mtype: Mtype::Invalid,
            content: String::new()
        }
    }
}

struct MessageFilter {
    pub min_id: usize,
    pub sender_cond: String,
    pub mtype_cond: Vec<Mtype>,
    pub contend_cond: String,
}

impl MessageFilter {
    pub fn matches(&self, msg: &Message) -> Option<Message>{
        let id_match = msg.id >= self.min_id;
        let type_match = self.mtype_cond.contains(&msg.mtype);
        let sender_match = true; // TODO: implement
        let all_match = type_match && sender_match && id_match;
        match all_match {
            true => Some(msg.clone()),
            false => None
        }
    }
}

struct ConnInterface {
    from_conn: Receiver<Message>,
    to_conn: Sender<Message>,
}

impl ConnInterface {
    pub fn get_msg(&mut self) -> Option<Message> {
        return match self.from_conn.try_recv() {
            Ok(msg) => Some(msg),
            Err(_) => None
        };
    }

}

const BUFFSIZE : usize  = 2048;

struct MessageChache {
    buffer: VecDeque<Message>,
}

impl MessageChache {
    pub fn new() -> Self {
        return Self{
            buffer: VecDeque::new(),
        };
    }
    
    pub fn log(&mut self, msg: Message) {
        self.buffer.push_back(msg);
        if self.buffer.len() > BUFFSIZE {
            self.buffer.pop_front();
        }
    }

    pub fn get_match(&mut self, filter: MessageFilter) -> Option<Message> {
        self.buffer.iter().find_map(|x| filter.matches(x))
    }
}

fn management_thread(new_conns: Receiver<ConnInterface>) {
    let mut conns : Vec<ConnInterface> = vec![];
    let mut cache = MessageChache::new();
    loop {
        // todo: implement culling of inactive connections for better processing times and avoiding mem leaks (over very long times)
        match new_conns.try_recv() {
            Ok(new_conn) => conns.push(new_conn),
            Err(_) => {},
        }

        for conn in conns.iter_mut() {
            if let Some(msg) = conn.get_msg() {
                // TODO: don't log requests, instead fullfill them
                cache.log(msg);
                println!("got message");
            }
        }


    }
}

fn main() {
    println!("hello world");
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let (manage_send, manage_recv) = channel::<ConnInterface>();

    thread::spawn(move || {management_thread(manage_recv);});

    for maybe_stream in listener.incoming() {
        let stream = match maybe_stream {
            Ok(s) => s,
            Err(_) => {continue;}
        };
        let (inc_sender, inc_receiver) = channel::<Message>();
        let (resp_sender, resp_receiver) = channel::<Message>();
        let conn_if = ConnInterface{
            from_conn: inc_receiver,
            to_conn: resp_sender
        };
        manage_send.send(conn_if).unwrap();
        thread::spawn(move || {handle_tcp_conn(stream, inc_sender, resp_receiver);});
    }
}
