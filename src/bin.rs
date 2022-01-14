use std::env;
use commentator::Tokenizer;
use commentator::spec::{self, Spec};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("filename required");
    }

    let mut reader = my_reader::BufReader::open(args[1].as_str())?;
    let mut buffer = String::new();
    let spec = spec::Java::new();
    let mut tkn = Tokenizer::new(spec);
    while let Ok(num) = reader.read_line(&mut buffer) {
        if num == 0 {
            break;
        }
        let line = buffer.as_str();
        tkn.update(num, line)
    }
    
    while let Some(cmt) = tkn.take() {
        println!("{}:{}: '{}'", cmt.line, cmt.start, cmt.text);
    }

    Ok(())
}


// https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
        cnt: usize,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader, cnt: 0 })
        }

        pub fn read_line(&mut self, buffer: &mut String) -> io::Result<usize> {
            buffer.clear();
            self.cnt += 1;
            match self.reader.read_line(buffer) {
                Ok(size) => Ok(if size > 0 { self.cnt } else { 0 }),
                Err(why) => Err(why)
            }
        }
    }
}
