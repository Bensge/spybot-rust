use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct TSChannel {
    id: i32,
    name: char,
}

#[derive(Debug, PartialEq, Eq)]
struct TSChannel1{
    id: i32,
    name: char,
}