use std::env;
use commentator::Tokenizer;
use commentator::spec::{self, Spec};
use output::Writer;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("filename required");
    }

    let mut reader = my_reader::BufReader::open(args[1].as_str())?;
    let mut buffer = String::new();
    let spec = spec::Java::new();
    let mut tkn = Tokenizer::new(&spec);
    let mut out = output::JSON::new();
    while let Ok(num) = reader.read_line(&mut buffer) {
        if num == 0 {
            break;
        }
        let line = buffer.as_str();
        tkn.update(num, line);
        while let Some(mut cmt) = tkn.take() {
            cmt.trim(&spec);
            out.write(cmt);
        }
    }
    tkn.finish();
    if let Some(mut cmt) = tkn.take() {
        cmt.trim(&spec);
        out.write(cmt);
    }
    out.flush();

    Ok(())
}

mod output {
    use commentator::Comment;
    use json;

    pub trait Writer {
        fn new() -> Self;
        fn write(&mut self, cmt: Comment);
        fn flush(&self);
    }

    pub struct Plain {
        buf: String
    }

    pub struct JSON {
        arr: json::JsonValue,
    }

    impl Writer for Plain {
        fn new() -> Self {
            Plain{buf: String::new()}
        }

        fn write(&mut self, cmt: Comment) {
            self.buf.push_str(format!("{}:{}: '{}'", cmt.line, cmt.start, cmt.text).as_str());
        }

        fn flush(&self) {
            println!("{}", self.buf);
        }
    }

    impl Writer for JSON {
        fn new() -> Self {
           JSON{arr: json::JsonValue::new_array()}
        }

        fn write(&mut self, cmt: Comment) {
            let item = json::object!{
                "line" => cmt.line,
                "start" => cmt.start,
                "body" => cmt.text,
            };
            if let Err(e) = self.arr.push(item) {
                panic!("Error: {}", e);
            }
        }

        fn flush(&self) {
            println!("{}", self.arr);
        }
    }
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
