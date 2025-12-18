use std::io::{self, Read};

#[derive(Debug, Clone)]
pub enum Value {
    SimpleString(String),
    BulkString(String),
    Array(Vec<Value>),
}

pub struct RespHandler {
    pub(crate) buffer: Vec<u8>,
    pub(crate) cursor: usize,
}

impl RespHandler {
    pub fn new(mut stream: impl Read) -> io::Result<Self> {
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;
        
        Ok(RespHandler {
            buffer: buffer[..bytes_read].to_vec(),
            cursor: 0,
        })
    }

    pub fn parse(&mut self) -> Option<Value> {
        let val = self.parse_value()?;
        Some(val)
    }

    fn parse_value(&mut self) -> Option<Value> {
        match self.buffer.get(self.cursor) {
            Some(&b'+') => self.parse_simple_string(),
            Some(&b'$') => self.parse_bulk_string(),
            Some(&b'*') => self.parse_array(),
            _ => None,
        }
    }

    fn parse_simple_string(&mut self) -> Option<Value> {
        self.cursor += 1; 
        
        let end = self.get_line_end()?;
        let s = String::from_utf8_lossy(&self.buffer[self.cursor..end]).to_string();
        
        self.cursor = end + 2;
        Some(Value::SimpleString(s))
    }

    fn parse_bulk_string(&mut self) -> Option<Value> {
        self.cursor += 1;
        
        let len_end = self.get_line_end()?;
        let len_str = String::from_utf8_lossy(&self.buffer[self.cursor..len_end]);
        let len: usize = len_str.parse().ok()?;
        
        self.cursor = len_end + 2; 
        
        let str_end = self.cursor + len;
        if str_end > self.buffer.len() { return None; }
        
        let s = String::from_utf8_lossy(&self.buffer[self.cursor..str_end]).to_string();
        
        self.cursor = str_end + 2; 
        Some(Value::BulkString(s))
    }

    fn parse_array(&mut self) -> Option<Value> {
        self.cursor += 1;
        
        let len_end = self.get_line_end()?;
        let len_str = String::from_utf8_lossy(&self.buffer[self.cursor..len_end]);
        let len: usize = len_str.parse().ok()?;
        
        self.cursor = len_end + 2;
        
        let mut items = Vec::new();
        for _ in 0..len {
            let val = self.parse_value()?;
            items.push(val);
        }
        
        Some(Value::Array(items))
    }

    fn get_line_end(&self) -> Option<usize> {
        for i in self.cursor..self.buffer.len() {
            if self.buffer[i] == b'\r' && i + 1 < self.buffer.len() && self.buffer[i+1] == b'\n' {
                return Some(i);
            }
        }
        None
    }
}


impl Value {
    pub fn serialize(self) -> String {
        match self {
            Value::SimpleString(s) => format!("+{}\r\n", s),
            Value::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
            Value::Array(_) => panic!("Value cannot be serialized to string for now"), 
        }
    }
}