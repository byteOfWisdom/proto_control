// use std::env;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::collections::VecDeque;
use std::ffi::{CStr, c_char};


// macro to return none when assigning an expression returning an option
macro_rules! return_none {
    ($x:expr) => {
        match $x {
            Ok(y) => y,
            Err(_) => {return None;},
        }
    };
}

macro_rules! rto {
    ($x:expr) => {
        match $x {
            Ok(y) => Some(y),
            Err(_) => None,
        }
    }
}


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
                    dbg!(resp.clone().as_string());
                    sock_writer.write_all(resp.as_string().as_bytes()).unwrap();
                    sock_writer.flush().unwrap();
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
    let sender = chunks.next()?;
    let mtype = str_to_mtype(chunks.next()?);
    let content = chunks.next()?;

    return Some(Message{
        id: 0,
        from: String::from(sender),
        mtype: mtype,
        content: String::from(content).replace("\0", "")
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

// TODO: add mtype QueryWait. set it to block until matching packet has been found
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

unsafe extern "C" {
    fn fnmatch(pattern: *const c_char, string: *const c_char, flags: i32) -> i32;
}


fn match_filter(filter: &str, test_string: &str) -> bool {
    dbg!(filter);
    dbg!(test_string);
    let sanitized_test_string = String::from(test_string) + "\0";
    let cstr_filter: &CStr = CStr::from_bytes_until_nul(filter.as_bytes()).unwrap();
    let cstr_test: &CStr = CStr::from_bytes_until_nul(sanitized_test_string.as_bytes()).unwrap();
    unsafe {
        return fnmatch(cstr_filter.as_ptr(), cstr_test.as_ptr(), 0) == 0;
    }
}

#[derive(Debug, Clone)]
struct MessageFilter {
    pub min_id: u64,
    pub sender_cond: String, //TODO: this has to be vec<string>
    pub mtype_cond: Vec<Mtype>, //TODO: this actually doesn't have to be vec
    pub content_cond: String,
}

impl MessageFilter {
    pub fn matches(&self, msg: &Message) -> Option<Message>{
        dbg!(self);
        let id_match = msg.id >= self.min_id;
        let type_match = self.mtype_cond.contains(&msg.mtype);
        let sender_match = match_filter(self.sender_cond.as_str(), msg.from.as_str());
        let content_match = match_filter(self.content_cond.as_str(), msg.content.as_str());
        let all_match = type_match && sender_match && id_match && content_match;
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
            content_cond: String::new(),
        }
    }

    pub fn assign(&mut self, key: &str, value: &str) {
        // ENSURE THAT ALL FILTER STRING HAVE NULL TERMINATORS!!!!!!!!
        match key {
            "min_id" => {self.min_id = value.parse::<u64>().unwrap_or(0);},
            "sender_cond" => {self.sender_cond = String::from(value) + "\0";}
            "content_cond" => {self.content_cond = String::from(value) + "\0";}
            "mtype_cond" => {self.mtype_cond = vec![str_to_mtype(value)];}
            _ => {},
        };
    }

    pub fn from_message(msg: Message) -> Option<Self> {
        // ENSURE THAT ALL FILTER STRING HAVE NULL TERMINATORS!!!!!!!!
        dbg!(msg.clone());
        let content = msg.content;
        let mut res = Self::empty();
        for kv_pair in content.split(";") {
            let mut kv = kv_pair.split("=");
            let key = kv.next()?;
            let value = kv.next()?;
            dbg!(key, value);
            res.assign(key, value);
            dbg!(res.clone());
        }
        dbg!(res.clone());
        return Some(res);
    }
}

struct ConnInterface {
    from_conn: Receiver<Message>,
    to_conn: Sender<Option<Message>>,
}

impl ConnInterface {
    pub fn get_msg(&mut self) -> Option<Message> {
        return rto!(self.from_conn.try_recv());
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
        if filter.min_id == 0 {
            self.buffer.iter().rev().find_map(|x| filter.matches(x))
        } else {
            self.buffer.iter().find_map(|x| filter.matches(x))
        }
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
                    dbg!("got a query!");
                    let filter = MessageFilter::from_message(msg).unwrap_or(MessageFilter::empty());
                    dbg!(filter.clone());
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
