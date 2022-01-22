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
/// let cmt = t.take();
/// assert!(cmt.is_some());
/// assert_eq!(cmt.unwrap().text, "\n * Entry point.\n ");
/// assert!(t.take().is_none());
/// ```
pub struct Tokenizer<S> where S: Spec {
    spec: S,
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
    complete: bool,
}

impl Comment {
    fn new() -> Self {
        Comment{
            text: String::new(),
            line: 0, start: 0, inline: false, complete: false,
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
}

impl<S: Spec> Tokenizer<S> {
    pub fn new(spec: S) -> Self {
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
                    self.last.begin(line, cnt+o);
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
