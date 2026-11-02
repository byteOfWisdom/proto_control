// use std::env;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::collections::VecDeque;


// the main function to handle a single socket
// only does the sending and receiving part, no actual logic (filtering, routing etc.) occurs
fn handle_tcp_conn(sock: TcpStream, inc_dump: Sender<Message>, response: Receiver<Option<Message>>) {
    // TODO: decice if unwrapping is acceptable or other error handling should be done
    let mut sock_reader = BufReader::new(sock.try_clone().unwrap());
    let mut sock_writer = BufWriter::new(sock.try_clone().unwrap());

    loop {
        let mut buffer: Vec<u8> = vec![];
        let n_chars = sock_reader.read_until(b'\0', &mut buffer);
        if n_chars.unwrap() > 0 {
            let msg_string = String::from_utf8(buffer).unwrap();
            let msg = parse_message(msg_string).unwrap_or(Message::new_empty());


            dbg!("got message: {}", msg.clone());
            match msg.mtype {
                Mtype::Invalid => {},
                Mtype::Query => {
                    inc_dump.send(msg).unwrap();
                    let resp = response.recv().unwrap()
                        .unwrap_or(Message::new_empty());
                    // TODO: don't discard/unwrap the write_all result
                    sock_writer.write_all(resp.as_string().as_bytes()).unwrap();
                },

                // TODO: don't make this an unwrap; handle gracefully
                _ => {inc_dump.send(msg).unwrap();}
            };

            // println!("got message");
        }
    }
}


fn parse_message(raw_message: String) -> Option<Message> {
    let mut chunks = raw_message.split("|");
    let sender = match chunks.next() {
        Some(s) => s,
        None => {return None;},
    };
    let mtype = match chunks.next() {
        Some(t) => str_to_mtype(t),
        None => {return None;},
    };
    let content = match chunks.next() {
        Some(c) => c,
        None => {return None;},
    };

    return Some(Message{
        id: 0,
        from: String::from(sender),
        mtype: mtype,
        content: String::from(content)
    });

}

fn str_to_mtype(type_str: &str) -> Mtype {
    match type_str {
        "Q" => Mtype::Query,
        "D" => Mtype::Data,
        "C" => Mtype::Command,
        "M" => Mtype::Misc,
        _ => Mtype::Invalid,
    }
}

fn mtype_to_string(mtype: Mtype) -> String {
    match mtype {
        Mtype::Query => String::from("Q"),
        Mtype::Data => String::from("D"),
        Mtype::Command => String::from("C"),
        Mtype::Misc => String::from("M"),
        Mtype::Invalid => String::from("I")
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Mtype {
    Data,
    Query,
    Command,
    Misc,
    Invalid
}

#[derive(Clone, Debug)]
struct Message {
    pub id: u64,
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

    pub fn as_string(&self) -> String {
        return format!("{}|{}|{}|{}\0", self.from, mtype_to_string(self.mtype), self.id, self.content);
    }
}

struct MessageFilter {
    pub min_id: u64,
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

    pub fn empty() -> Self {
        Self {
            min_id: u64::MAX,
            sender_cond: String::new(),
            mtype_cond: vec![Mtype::Invalid],
            contend_cond: String::new(),
        }
    }

    pub fn from_message(msg: Message) -> Option<Self> {
        // TODO: implement conversion of message to filter
        return None;
    }
}

struct ConnInterface {
    from_conn: Receiver<Message>,
    to_conn: Sender<Option<Message>>,
}

impl ConnInterface {
    pub fn get_msg(&mut self) -> Option<Message> {
        return match self.from_conn.try_recv() {
            Ok(msg) => Some(msg),
            Err(_) => None
        };
    }

    pub fn reply(&mut self, msg: Option<Message>) {
        _ = self.to_conn.send(msg);
    }

}

const BUFFSIZE : usize  = 2048;

struct MessageChache {
    buffer: VecDeque<Message>,
    current_id: u64,
}

impl MessageChache {
    pub fn new() -> Self {
        return Self{
            buffer: VecDeque::new(),
            current_id: 0,
        };
    }
    
    pub fn log(&mut self, mut msg: Message) {
        self.current_id += 1;
        msg.id = self.current_id;
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
        // TODO: implement culling of inactive connections for better processing times and avoiding mem leaks (over very long times)
        match new_conns.try_recv() {
            Ok(new_conn) => conns.push(new_conn),
            Err(_) => {},
        }

        for conn in conns.iter_mut() {
            if let Some(msg) = conn.get_msg() {
                // TODO: don't log requests, instead fullfill them
                if msg.mtype == Mtype::Query {
                    let filter = MessageFilter::from_message(msg).unwrap_or(MessageFilter::empty());
                    let resp = cache.get_match(filter);
                    conn.reply(resp);
                }
                else {
                    cache.log(msg);
                }
                println!("got message");
            }
        }


    }
}

fn main() {
    // TODO: implement config via commandline args
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let (manage_send, manage_recv) = channel::<ConnInterface>();

    thread::spawn(move || {management_thread(manage_recv);});

    for maybe_stream in listener.incoming() {
        let stream = match maybe_stream {
            Ok(s) => s,
            Err(_) => {continue;}
        };
        let (inc_sender, inc_receiver) = channel::<Message>();
        let (resp_sender, resp_receiver) = channel::<Option<Message>>();
        let conn_if = ConnInterface{
            from_conn: inc_receiver,
            to_conn: resp_sender
        };
        manage_send.send(conn_if).unwrap();
        thread::spawn(move || {handle_tcp_conn(stream, inc_sender, resp_receiver);});
    }
}
