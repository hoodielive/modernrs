#![allow(dead_code)]
#[derive(Debug)]
pub enum IpAddr
{
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
pub enum Message
{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}
