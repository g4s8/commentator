
/// Comment specification.
/// Detects comment for different languages.
/// Use implementations:
///  - `Java::new()` - for C-style comment languages (C, C++, Java, Go, Rust, JS, etc)
///  - `Bash::new()` - for `#` comments detector (bash, shell, python, etc)
///  - `HTML::new()` - for markup languages (HTML, XML, etc)
pub trait Spec {
    fn new() -> Self;

    fn is_begin(&self, src: &str) -> Option<usize>;

    fn is_end(&self, src: &str) -> Option<usize>;

    fn is_inline(&self, src: &str) -> Option<usize>;

    fn trim<'a>(&self, src: &'a str) -> &'a str;
}

pub struct Java {}
pub struct HTML {}
pub struct Bash {}

impl Spec for Java {
    fn new() -> Java{
        Java{}
    }

    fn is_begin(&self, src: &str) -> Option<usize> {
        if src.starts_with("/**") {
            Some(3)
        } else if src.starts_with("/*") {
            Some(2)
        } else {
            None
        }
    }

    fn is_end(&self, src: &str) -> Option<usize> {
        if src.starts_with("*/") {
            Some(2)
        } else {
            None
        }
    }

    fn is_inline(&self, src: &str) -> Option<usize> {
        if src.starts_with("//") {
            Some(2)
        } else {
            None
        }
    }

    fn trim<'a>(&self, src: &'a str) -> &'a str {
        if src.starts_with("*") {
            src[1..].as_ref()
        } else if src.starts_with("//") {
            src[2..].as_ref()
        } else {
            src
        }
    }
}

impl Spec for HTML {
    fn new() -> HTML{
        HTML{}
    }

    fn is_begin(&self, src: &str) -> Option<usize> {
        if src.starts_with("<!--") {
            Some(4)
        } else {
            None
        }
    }

    fn is_end(&self, src: &str) -> Option<usize> {
        if src.starts_with("-->") {
            Some(3)
        } else {
            None
        }
    }

    fn is_inline(&self, _: &str) -> Option<usize> {
        None
    }

    fn trim<'a>(&self, src: &'a str) -> &'a str {
        src
    }
}

impl Spec for Bash {
    fn new() -> Bash {
        Bash{}
    }

    fn is_begin(&self, _: &str) -> Option<usize> {
        None
    }

    fn is_end(&self, _: &str) -> Option<usize> {
        None
    }

    fn is_inline(&self, src: &str) -> Option<usize> {
        if src.starts_with("#") {
            Some(1)
        } else {
            None
        }
    }

    fn trim<'a>(&self, src: &'a str) -> &'a str {
        if src.starts_with("#") {
            src[1..].as_ref()
        } else {
            src
        }
    }
}
