use std::collections::HashMap;

use super::data::{Node,Inner};
use quick_xml::reader::Reader;
use quick_xml::events::{Event};

pub fn read_xml(text:&str)->Node{
    let mut reader = Reader::from_str(&text);
    reader.trim_text(true);
    let mut root = Node::new();
    dfs(&mut reader,&mut root);
    root
}

fn dfs(reader:&mut Reader<&[u8]>,node:&mut Node){
    let mut buf = Vec::new();
    let mut inner = Inner::MAP(HashMap::new());
    loop {
        match reader.read_event_into(&mut buf){
            Ok(Event::Start(key))=> {
                let mut p = Node::new();
                let value = String::from_utf8_lossy(key.name().into_inner()).into_owned();
                dfs(reader,&mut p);
                if inner.contains(&value){
                    inner = inner.convert_vec();
                }
                inner.insert(value, p);
            },
            Ok(Event::End(_)) =>{
                node.inner = inner;
                return;
            }
            Ok(Event::Text(text)) =>{
                let value = String::from_utf8_lossy(&*text.into_inner()).into_owned();
                node.value = value;
            }
            Ok(Event::Eof) => {
                node.inner = inner;
                return;
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ =>()
        }
    }
}