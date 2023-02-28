use std::collections::HashMap;
use std::fmt::{Debug,Formatter};
use std::ops::Index;

pub struct Node{
    pub value: String,
    pub inner: Inner,
}

#[derive(Debug)]
pub enum Inner {
    VEC(Vec<Node>),
    MAP(HashMap<String,Node>)
}
// pub trait Node<T>{
//     fn insert(&mut self,k:String,v:T);
//     fn set_inner(&mut self,inner:T);
// }
impl Inner{
    pub fn insert(&mut self,k:String,v:Node){
        match self{
            Inner::VEC(ref mut inner)=>inner.push(v),
            Inner::MAP(ref mut inner) => {inner.insert(k, v);},
        }
    }

    pub fn contains(&self,k:&str)->bool{
        match self{
            Inner::MAP(inner)=>inner.contains_key(k),
            _=>false
        }
    }

    pub fn convert_vec(self) -> Inner{
        match self{
            Inner::MAP(data) => Inner::VEC(data.into_values().collect()),
            _ => self
        }
    }
}
impl Node{
    pub fn new()->Self{
        Self { value:"".to_string(), inner: Inner::MAP(HashMap::new()) }
    }
}

impl Debug for Node{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        f.debug_tuple("")
        .field(&self.value)
        .field(&self.inner)
        .finish()
    }
}