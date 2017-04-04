use std::vec::Vec;

#[derive(Debug)]
pub enum Node {
    Begin(Vec<Box<Node>>),
    Integer(String),
    String(String),
    Send(Box<Node>, String, Vec<Box<Node>>)
}
