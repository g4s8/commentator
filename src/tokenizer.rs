use crate::spec::Spec;

use std::{
    vec::Vec,
    option::Option,
};
use unicode_segmentation::UnicodeSegmentation;

/// Tokenizer for comments.
/// It accepts lines of code as input and generates
/// comment structures as output.
///
/// # Examples
///
/// ```
/// use crate::spec;
///
/// let mut t = Tokenizer::new(spec::Java::new());
/// t.update(1, "/*\n");
/// t.update(2, " * Entry point.\n");
/// t.update(3, " */\n");
/// t.update(4, "public static void main(String... args) {\n");
/// t.update(5, "  System.out.println(\"hello world\");\n");
/// t.update(6, "}\n");
/// t.finish();
/// let cmt = t.take();
/// assert!(cmt.is_some());
/// assert_eq!(cmt.unwrap().text, "\n * Entry point.\n ");
/// assert!(t.take().is_none());
/// ```
pub struct Tokenizer<'a, S> where S: Spec {
    spec: &'a S,
    result: Vec<Comment>,
    comment: bool,
    last: Comment,
}

#[derive(Clone)]
pub struct Comment {
    pub line: usize,
    pub start: usize,
    pub text: String,
    inline: bool,
}

fn lines_ws_offset(lines: &Vec<&str>) -> usize {
    let mut lo = 0;
    for (n, l) in lines.iter().enumerate() {
        let mut cnt = 0;
        while l[cnt..].starts_with(" ") {
            cnt += 1;
        }
        if n == 0 || cnt < lo {
            lo = cnt;
        }
        if lo == 0 {
            break;
        }
    }
    lo
}

impl Comment {
    fn new() -> Self {
        Comment{
            text: String::new(),
            line: 0, start: 0, inline: false,
        }
    }

    fn begin(&mut self, line: usize, start: usize) {
        self.text.clear();
        self.line = line;
        self.start = start;
        self.inline = false;
    }

    fn write(&mut self, buf: &str) {
        self.text.push_str(buf);
    }

    pub fn trim<'a, S: Spec>(&mut self, spec: &'a S) {
        let mut lines = Vec::new();

        // 1) split comment by newlines, skip empty or whitespace lines
        // in the beginning, remove trailing whitespaces and add lines to
        // array.
        for l in self.text.split("\n") {
            if lines.is_empty() && l.is_empty() {
                // skip first empty lines
                continue;
            }
            lines.push(l.trim_end());
        }

        // 2) remove empty or whitespace lines from the end
        while let Some(l) = lines.last() {
            if l.is_empty() {
                lines.truncate(lines.len() - 1);
            } else {
                break;
            }
        }

        // 3) calculate if we the comment is canonical: if whitespace count on
        // each line is bigger that offset of openning comment literal, than we
        // can trim these whitespaces. If any line doesn't have enough spaces, then
        // we can't trim the whole comment. If we can - then remove whitespaces from
        // the beginning of line.
        let offset = lines_ws_offset(&lines);
        if offset > 0 {
            for l in lines.iter_mut() {
                *l = l[offset..].as_ref();
            }
        }

        // 4) trim based on language comments specification, e.g. remove `*` chars
        // for Java comments.
        for l in lines.iter_mut() {
            *l = spec.trim(l);
        }

        // 5) calculate minimal offset of whitespaces for each line and trim each line by
        // this offset if it's > 0.
        let offset = lines_ws_offset(&lines);
        if offset > 0 {
            for l in lines.iter_mut() {
                *l = l[offset..].as_ref();
            }
        }
        self.text = lines.join("\n");
    }
}

impl<'a, S: Spec> Tokenizer<'a, S> {
    pub fn new(spec: &'a S) -> Self {
        let r = Vec::new();
        Tokenizer{
            spec,
            result: r,
            comment: false,
            last: Comment::new(),
        }
    }


    pub fn update(&mut self, line: usize, buf: &str) {
        let mut iter = buf.grapheme_indices(true);
        let mut cnt = 0;
        loop {
            let tail = iter.as_str();
            if !self.comment {
                // if we are not in comment scope and entering multi-line comment
                // then begin new comment.
                if let Some(o) = self.spec.is_begin(tail) {
                    self.comment = true;
                    self.last.begin(line, cnt);
                    cnt += o;
                    for _ in 1..o {
                        iter.next();
                    }
                    if let None = iter.next() {
                        break;
                    }
                    continue;
                }
                if let Some(o) = self.spec.is_inline(tail) {
                    if !self.last.inline {
                        // if we are not in comment scope and entering a single-line comment
                        // then begin new comment, write the rest of line, and exit loop.
                        self.last.begin(line, cnt+o);
                        self.last.inline = true;
                    } else {
                        // we continue previous single-line comment
                        self.result.remove(0);
                    }
                    self.last.write(tail[o..].as_ref());
                    self.result.insert(0, self.last.clone());
                    break;
                }
            }
            if self.comment {
                let mut sub = tail.grapheme_indices(true);
                let mut sub_cnt = 0;
                loop {
                    let sub_ref = sub.as_str();
                    if let Some(o) = self.spec.is_end(sub_ref) {
                        if let Some(offset) = tail.find(sub_ref) {
                            self.last.write(tail[..offset].as_ref());
                        } else {
                            panic!("string tail is not a substring!Oo");
                        }
                        self.result.insert(0, self.last.clone());
                        self.comment = false;
                        for _ in 1..o {
                            iter.next();
                        }
                        cnt += sub_cnt+o;
                        break;
                    }
                    sub_cnt += 1;
                    if let None = sub.next() {
                        break;
                    }
                }
                if !self.comment {
                    if let None = iter.next() {
                        break;
                    }
                    continue;
                }
                // no comment end found on this line
                self.last.write(tail);
                // println!("debug: line ended with opened comment: {}", self.last.to_str());
                break;
            }
            if let None = iter.next() {
                break;
            }
            cnt += 1;
        }
    }

    // WARNING: this method panics on non-ASCII symbol
    pub fn update_ascii(&mut self, line: usize, buf: &str) {
        let mut i = 0;
        while i < buf.len() {
            let tail = buf[i..].as_ref();
            // println!("debug: comment={} tail: {}", self.comment, tail);
            if !self.comment {
                // if we are not in comment scope and entering multi-line comment
                // then begin new comment.
                if let Some(o) = self.spec.is_begin(tail) {
                    // println!("debug: comment found at {}", tail);
                    self.comment = true;
                    self.last.begin(line, i+o);
                    i += o;
                    continue;
                }
                if let Some(o) = self.spec.is_inline(tail) {
                    // println!("debug: inline comment found at {}", tail);
                    if !self.last.inline {
                        // if we are not in comment scope and entering a single-line comment
                        // then begin new comment, write the rest of line, and exit loop.
                        self.last.begin(line, i+o);
                        self.last.inline = true;
                    } else {
                        // we continue previous single-line comment
                        self.result.pop();
                    }
                    self.last.write(buf[i+o..].as_ref());
                    self.result.insert(0, self.last.clone());
                    break;
                }
            }
            if self.comment {
                for sub in 0..tail.len() {
                    let sub_ref = tail[sub..].as_ref();
                    // println!("debug: check comment end: {}", sub_ref);
                    if let Some(o) = self.spec.is_end(sub_ref) {
                        self.last.write(tail[..sub].as_ref());
                        self.result.insert(0, self.last.clone());
                        // println!("debug: comment found: {}", self.last);
                        self.comment = false;
                        i += sub+o;
                        // continue 'MAIN;
                        break;
                    }
                }
                if !self.comment {
                    continue;
                }
                // no comment end found on this line
                self.last.write(tail);
                // println!("debug: line ended with opened comment: {}", self.last);
                break;
            }
            i += 1;
        }
    }

    pub fn finish(&mut self) {
        self.result.insert(0, Comment::new())
    }

    pub fn take(&mut self) -> Option<Comment> {
        if self.result.len() > 1 {
            return self.result.pop();
        }
        None
    }
}
