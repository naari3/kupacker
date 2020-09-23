extern crate glob;

use glob::glob;

use std::io::Read;
use std::path::Path;

use std::fs::File;
use std::io::Write;

#[derive(Debug, PartialEq)]
struct Kupack {
    header: KupackHeader,
    files: Vec<KupackFile>,
}

#[derive(Debug, PartialEq)]
struct KupackFile {
    name: String,
    body: Vec<u8>,
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

pub fn pack_cmd(file_path: &str, input_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let kupack_files = glob_targets(Path::new(input_dir).join("*").to_str().unwrap());
    let kupack = Kupack::new_with_kupack_files(kupack_files);

    let mut file = File::create(file_path)?;
    file.write_all(&kupack.as_bytes())?;
    file.flush()?;

    Ok(())
}

impl Kupack {
    pub fn new_with_kupack_files(kupack_files: Vec<KupackFile>) -> Kupack {
        Kupack {
            header: KupackHeader::new_with_kupack_files(&kupack_files),
            files: kupack_files,
        }
    }

    pub fn as_bytes(self) -> Vec<u8> {
        println!("write binaries...");
        let mut header = self.header.as_bytes();
        let mut bodies = self
            .files
            .into_iter()
            .flat_map(|f| {
                println!(
                    "write: {}, filesize: {} byte",
                    &f.name,
                    &f.body.len()
                );
                f.body
            })
            .collect::<Vec<_>>();
        header.append(&mut bodies);
        header
    }
}

impl KupackHeader {
    pub fn new_with_kupack_files(kupack_files: &Vec<KupackFile>) -> KupackHeader {
        println!("calculate offsets...");
        let file_count = kupack_files.len() as u64;
        let offset = kupack_files.iter().fold(0u64, |mut sum, f| {
            sum += f.name.len() as u64;
            sum
        })
            // all filename length
            + file_count * (2 * 10 + 2 + 2)
            + 10 + 4 + 4;
        // file size, offset, brackets, comma. ex: (filename,0001723944,0000011890)
        // and first two brackets ( offsets(10 digits) ) ( file counts(4 digits) )
        let mut file_details = Vec::with_capacity(file_count as usize);
        let mut offset_for_each_detail = offset;
        for f in kupack_files.into_iter() {
            let detail = FileDetail {
                name: f.name.to_string(),
                size: f.body.len() as u64,
                offset: offset_for_each_detail,
            };
            file_details.push(detail);
            offset_for_each_detail += f.body.len() as u64;
        }
        KupackHeader {
            file_count,
            offset,
            file_details,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let top = format!("({:>010})({:>04})", self.offset, self.file_count);
        bytes.append(&mut top.as_bytes().to_vec());
        let details = self
            .file_details
            .iter()
            .map(|fd| format!("({:},{:>010},{:>010})", fd.name, fd.size, fd.offset))
            .collect::<Vec<String>>()
            .join("");
        bytes.append(&mut details.as_bytes().to_vec());
        bytes
    }
}

fn glob_targets(target_dir: &str) -> Vec<KupackFile> {
    let mut files = Vec::new();
    for entry in glob(target_dir).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                files.push(KupackFile {
                    body: read_file(&path),
                    name: format!("{}", path.file_name().unwrap().to_str().unwrap()),
                });
                println!("read: {:?}", path);
            }
            Err(e) => println!("{:?}", e),
        }
    }
    files
}

fn read_file<P: AsRef<Path>>(file_path: P) -> Vec<u8> {
    let mut file = std::fs::File::open(file_path).expect("file open failed");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("file read failed");
    buf
}
