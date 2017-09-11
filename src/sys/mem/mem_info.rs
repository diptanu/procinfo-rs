//! Process meminfo information from `/proc/meminfo`

use std::fs::File;
use std::io::Result;

use nom::{IResult, line_ending, multispace, not_line_ending, space};


use parsers::{
    map_result,
    parse_line,
    read_to_end
};


/// MemInfo information
#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct MemInfo {
    pub total: usize,
    pub free: usize,
    pub available: usize,
    pub buffers: usize,
    pub cached: usize,
    pub swap_cached: usize,
    pub active: usize,
    pub inactive: usize,
    pub high_total: usize,
    pub high_free: usize,
    pub low_total: usize,
    pub low_free: usize,
    pub swap_total: usize,
    pub swap_free: usize,
}


named!(parse_total<String>,   delimited!(tag!("MemTotal:\t"),      parse_line,         line_ending));
named!(parse_free<String>,   delimited!(tag!("MemFree:\t"),      parse_line,         line_ending));
named!(parse_available<String>,   delimited!(tag!("MemAvailable:\t"),      parse_line,         line_ending));
named!(parse_buffers<String>,   delimited!(tag!("Buffers:\t"),      parse_line,         line_ending));
named!(parse_cached<String>,   delimited!(tag!("Cached:\t"),      parse_line,         line_ending));
named!(parse_swap_cached<String>,   delimited!(tag!("SwapCached:\t"),      parse_line,         line_ending));

fn parse_meminfo(input: &[u8]) -> IResult<&[u8], MemInfo> {
    let mut mem_info: MemInfo = Default::default();
    map!(input,
         many0!(
             alt!(parse_total => { |value| mem_info.total = value}
                | parse_free  => { |value| mem_info.free = value}
                | parse_available => { |value| mem_info.available = value}
                | parse_buffers => { |value| mem_info.buffers = value}
                | parse_cached => { |value| mem_info.cached = value}
                | parse_swap_cached = { |value| mem_info.swap_cached = value}
             )
         ),
        { |_| { mem_info }})
}

fn mem_info_file(file: &mut File) -> Result<MemInfo> {
    let mut buf = Vec::with_capacity(file.len());
    map_result(parse_meminfo(try!(read_to_end(file, &mut buf))))
}

pub fn mem_info() -> Result<MemInfo> {
    mem_info_file(&mut try!(File::open("/proc/meminfo")))
}
