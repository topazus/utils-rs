use std::io::BufRead;
use std::io::Read;
use std::{
    fs::{self, File},
    os,
};
mod cpu;
mod disk;
mod file;
mod filesystem;
mod re;
fn main() {
    let contents = std::fs::read_to_string("/proc/cpuinfo").unwrap();
    let mut lines = contents.lines().collect::<Vec<_>>();
    lines.retain(|&x| x != "");
    println!("{:?}", lines);
}
