use commentator::Tokenizer;
use commentator::spec::StandardSpec;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut format = String::new();
    let mut lang = String::new();
    let mut trim = false;
    let mut file = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Extract comments from source file");
        ap.refer(&mut format)
            .add_option(&["--format"], Store, "output format (plain|json)");
        ap.refer(&mut lang)
            .add_option(&["--lang"], Store, "comment language specification: (java|bash|html)");
        ap.refer(&mut trim)
            .add_option(&["--trim"], StoreTrue, "trim comments flag");
        ap.refer(&mut file)
            .add_argument("file", Store, "file to parse");
        ap.parse_args_or_exit();
    }

    let spec = match lang.as_str() {
        "c" | "java" | "go" | "cpp "=> StandardSpec::C,
        "bash" | "sh" | "ruby" => StandardSpec::Bash,
        "html" | "xml" => StandardSpec::HTML,
        "rust" => StandardSpec::Rust,
        _ => panic!("unknown language type: `{}`", lang),
    };

    let mut reader = my_reader::BufReader::open(file.as_str())?;
    let mut buffer = String::new();
    let mut tkn = Tokenizer::new(&spec);
    let mut out = output::Format::from_str(format.as_str()).unwrap();
    out.begin();
    let mut pos = 0;
    while let Ok(num) = reader.read_line(&mut buffer) {
        if num == 0 {
            break;
        }
        let line = buffer.as_str();
        tkn.update(num, line);
        while let Some(mut cmt) = tkn.take() {
            if trim {
                cmt.trim(&spec);
            }
            out.write(pos, cmt);
            pos += 1;
        }
    }
    tkn.finish();
    if let Some(mut cmt) = tkn.take() {
        if trim {
            cmt.trim(&spec);
        }
        out.write(pos, cmt);
    }
    out.end();

    Ok(())
}

mod output {
    use commentator::Comment;
    use json;
    use std::{error::Error, fmt, str::FromStr};

    #[derive(Debug)]
    pub struct ParseFormatErr {
        msg: String
    }

    impl Error for ParseFormatErr {}

    impl fmt::Display for ParseFormatErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "format parse error: {}. Supported format options are `plain` or `json`",
                self.msg)
        }
    }

    pub enum Format {
        Plain,
        JSON,
    }

    impl Format {
        pub fn write(&mut self, pos: usize, cmt: Comment) {
            match self {
                Format::Plain => println!("[{}]{}:{}: '{}'", pos, cmt.line, cmt.start, cmt.text),
                Format::JSON => println!("{}{}", if pos > 0 { "," } else { "" }, json::object!{
                    "line" => cmt.line,
                    "start" => cmt.start,
                    "body" => cmt.text,
                }),
            }
        }

        pub fn begin(&self) {
            match self {
                Format::JSON => println!("["),
                _ => (),
            }
        }

        pub fn end(&self) {
            match self {
                Format::JSON => println!("]"),
                _ => (),
            }
        }
    }

    impl FromStr for Format {
        type Err = ParseFormatErr;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "json" => Ok(Format::JSON),
                "plain" => Ok(Format::Plain),
                _ => Err(ParseFormatErr{msg: format!("unknown option `{}`", s)})
            }
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
