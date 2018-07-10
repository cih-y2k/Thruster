// use std::fmt::{self, Write};

// use bytes::{BytesMut, BufMut};

// pub struct Response {
//     pub headers: Vec<(String, String)>,
//     pub response: Vec<u8>,
//     pub status_message: StatusMessage,
// }

// pub enum StatusMessage {
//     Ok,
//     Custom(u32, String)
// }

// impl Response {
//     pub fn new() -> Response {
//         Response {
//             headers: Vec::new(),
//             response: Vec::new(),
//             status_message: StatusMessage::Ok,
//         }
//     }

//     pub fn status_code(&mut self, code: u32, message: &str) -> &mut Response {
//         self.status_message = StatusMessage::Custom(code, message.to_string());
//         self
//     }

//     pub fn header(&mut self, name: &str, val: &str) -> &mut Response {
//         self.headers.push((name.to_string(), val.to_string()));
//         self
//     }

//     pub fn body(&mut self, s: &str) -> &mut Response {
//         self.response = s.as_bytes().to_vec();
//         self
//     }

//     pub fn body_bytes(&mut self, b: &[u8]) -> &mut Response {
//         self.response = b.to_vec();
//         self
//     }
// }

// pub fn encode(msg: Response, buf: &mut BytesMut) {
//     let length = msg.response.len();
//     let now = ::date::now();

//     write!(FastWrite(buf), "\
//         HTTP/1.1 {}\r\n\
//         Server: Example\r\n\
//         Content-Length: {}\r\n\
//         Date: {}\r\n\
//     ", msg.status_message, length, now).unwrap();

//     for &(ref k, ref v) in &msg.headers {
//         push(buf, k.as_bytes());
//         push(buf, ": ".as_bytes());
//         push(buf, v.as_bytes());
//         push(buf, "\r\n".as_bytes());
//     }

//     push(buf, "\r\n".as_bytes());
//     push(buf, msg.response.as_slice());
// }

// fn push(buf: &mut BytesMut, data: &[u8]) {
//     buf.reserve(data.len());
//     unsafe {
//         buf.bytes_mut()[..data.len()].copy_from_slice(data);
//         buf.advance_mut(data.len());
//     }
// }

// // TODO: impl fmt::Write for Vec<u8>
// //
// // Right now `write!` on `Vec<u8>` goes through io::Write and is not super
// // speedy, so inline a less-crufty implementation here which doesn't go through
// // io::Error.
// struct FastWrite<'a>(&'a mut BytesMut);

// impl<'a> fmt::Write for FastWrite<'a> {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         push(&mut *self.0, s.as_bytes());
//         Ok(())
//     }

//     fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
//         fmt::write(self, args)
//     }
// }

// impl fmt::Display for StatusMessage {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             StatusMessage::Ok => f.pad("200 OK"),
//             StatusMessage::Custom(c, ref s) => write!(f, "{} {}", c, s),
//         }
//     }
// }

use std::fmt::{self, Write};

use bytes::{BytesMut, BufMut};

pub struct Response {
    pub headers: Vec<(String, String)>,
    pub response: Vec<u8>,
    pub status_message: StatusMessage,
    pub header_raw: BytesMut
}

pub enum StatusMessage {
    Ok,
    Custom(u32, String)
}

impl Response {
    pub fn new() -> Response {
        Response {
            headers: Vec::new(),
            response: Vec::new(),
            status_message: StatusMessage::Ok,
            header_raw: BytesMut::new()
        }
    }

    pub fn status_code(&mut self, code: u32, message: &str) -> &mut Response {
        self.status_message = StatusMessage::Custom(code, message.to_string());
        self
    }

    pub fn header(&mut self, name: &str, val: &str) -> &mut Response {
        self.headers.push((name.to_string(), val.to_string()));
        self.header_raw.extend_from_slice(name.as_bytes());
        self.header_raw.extend_from_slice(": ".as_bytes());
        self.header_raw.extend_from_slice(val.as_bytes());
        self.header_raw.extend_from_slice("\r\n".as_bytes());
        self
    }

    pub fn header_raw(&mut self, buf: &BytesMut) -> &mut Response {
        self.header_raw.extend_from_slice(buf);

        self
    }

    pub fn body(&mut self, s: &str) -> &mut Response {
        self.response = s.as_bytes().to_vec();
        self
    }

    pub fn body_bytes(&mut self, b: &[u8]) -> &mut Response {
        self.response = b.to_vec();
        self
    }
}

pub fn encode(msg: Response, buf: &mut BytesMut) {
    let length = msg.response.len();
    let now = ::date::now();

    write!(FastWrite(buf), "\
        HTTP/1.1 {}\r\n\
        Server: Example\r\n\
        Content-Length: {}\r\n\
        Date: {}\r\n\
    ", msg.status_message, length, now).unwrap();

    buf.extend_from_slice(&msg.header_raw);
    buf.extend_from_slice("\r\n".as_bytes());
    buf.extend_from_slice(msg.response.as_slice());
}

// TODO: impl fmt::Write for Vec<u8>
//
// Right now `write!` on `Vec<u8>` goes through io::Write and is not super
// speedy, so inline a less-crufty implementation here which doesn't go through
// io::Error.
struct FastWrite<'a>(&'a mut BytesMut);

impl<'a> fmt::Write for FastWrite<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        (*self.0).extend_from_slice(s.as_bytes());
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        fmt::write(self, args)
    }
}

impl fmt::Display for StatusMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StatusMessage::Ok => f.pad("200 OK"),
            StatusMessage::Custom(c, ref s) => write!(f, "{} {}", c, s),
        }
    }
}