use std::io::BufRead;
use std::io::Read;
use std::{
    fs::{self, File},
    os,
};

use cpu::split_list;
mod cpu;
mod disk;
mod file;
mod filesystem;
mod re;

fn main() {
    let contents = std::fs::read_to_string("/proc/cpuinfo").unwrap();
    let mut lines = contents.lines().collect::<Vec<_>>();
    let res = split_list(lines, "");
    let c = res[0]
        .iter()
        .map(|&x| x.split(":").nth(0).unwrap().trim())
        .collect::<Vec<&str>>();
    c.iter()
        .for_each(|&x| println!("{}", x.replace(" ", "_").to_lowercase()));
}
