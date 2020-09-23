extern crate nom;

use nom::branch::permutation;
use nom::bytes::complete::{tag, take, take_till};
use nom::combinator::map_res;
use nom::multi::{count, separated_list};
use nom::sequence::delimited;
use nom::IResult;

use std::io::Read;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Kupack {
    header: KupackHeader,
    pub files: Vec<KupackFile>,
}

#[derive(Debug, PartialEq)]
pub struct KupackFile {
    pub name: String,
    pub body: Vec<u8>,
}

#[derive(Debug, PartialEq)]
struct KupackHeader {
    offset: u64,
    file_count: u64,
    file_details: Vec<FileDetail>,
}

#[derive(Debug, PartialEq)]
struct FileDetail {
    name: String,
    size: u64,
    offset: u64,
}

#[derive(Debug, PartialEq)]
enum Element {
    Number(u64),
    FileDetail(FileDetail),
}

pub fn read_file<P: AsRef<Path>>(file_path: P) -> Vec<u8> {
    let mut file = std::fs::File::open(file_path).expect("file open failed");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("file read failed");
    buf
}

pub fn kupack(input: &[u8]) -> IResult<&[u8], Kupack> {
    let (input, kupack_header) = kupack_header(input)?;
    let mut input = input.clone();
    let mut offset = kupack_header.offset.clone();

    let mut kupack_files = Vec::with_capacity(kupack_header.file_count as usize);
    for detail in kupack_header.file_details.iter() {
        let input_ = input.clone();
        let (input_, _) = take(offset - detail.offset)(input_)?;
        let (input_, binary) = take(detail.size)(input_)?;
        offset = detail.offset + detail.size;

        kupack_files.push(KupackFile {
            name: detail.name.to_string(),
            body: binary.to_vec(),
        });
        input = input_;
    }
    Ok((
        input,
        Kupack {
            files: kupack_files,
            header: kupack_header,
        },
    ))
}

fn kupack_header(input: &[u8]) -> IResult<&[u8], KupackHeader> {
    let (input, (offset, file_count)) = kupack_header_meta(input)?;
    let (input, details) = count(
        map_res(element, |e| -> Result<FileDetail, ()> {
            if let Element::FileDetail(detail) = e {
                Ok(detail)
            } else {
                Err(())
            }
        }),
        file_count as usize,
    )(input)?;
    Ok((
        input,
        KupackHeader {
            offset: offset,
            file_count: file_count,
            file_details: details,
        },
    ))
}

fn kupack_header_meta(input: &[u8]) -> IResult<&[u8], (u64, u64)> {
    permutation((
        map_res(element, |e| -> Result<u64, ()> {
            if let Element::Number(offset) = e {
                Ok(offset)
            } else {
                Err(())
            }
        }),
        map_res(element, |e| -> Result<u64, ()> {
            if let Element::Number(file_count) = e {
                Ok(file_count)
            } else {
                Err(())
            }
        }),
    ))(input)
}

fn element(input: &[u8]) -> IResult<&[u8], Element> {
    map_res(
        delimited(
            tag(b"("),
            separated_list(tag(b","), take_till(|c| c == b',' || c == b')')),
            tag(b")"),
        ),
        |strs: Vec<&[u8]>| -> Result<Element, std::num::ParseIntError> {
            if strs.len() == 1 {
                return Ok(Element::Number(
                    std::str::from_utf8(strs[0]).unwrap().parse()?,
                ));
            } else {
                return Ok(Element::FileDetail(FileDetail {
                    name: std::str::from_utf8(strs[0]).unwrap().to_string(),
                    size: std::str::from_utf8(strs[1]).unwrap().parse()?,
                    offset: std::str::from_utf8(strs[2]).unwrap().parse()?,
                }));
            }
        },
    )(input)
}

#[test]
fn element_test() {
    assert_eq!(
        element("(1234)".as_bytes()),
        Ok(("".as_bytes(), Element::Number(1234)))
    );
    assert_eq!(
        element("(test.wav,1234,5678)".as_bytes()),
        Ok((
            "".as_bytes(),
            Element::FileDetail(FileDetail {
                name: "test.wav".to_string(),
                size: 1234,
                offset: 5678
            })
        ))
    );
}
