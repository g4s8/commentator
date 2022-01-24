
/// Comment specification.
/// Detects comment for different languages.
/// Use implementations:
///  - `Java::new()` - for C-style comment languages (C, C++, Java, Go, Rust, JS, etc)
///  - `Bash::new()` - for `#` comments detector (bash, shell, python, etc)
///  - `HTML::new()` - for markup languages (HTML, XML, etc)
pub trait Spec {
    fn is_begin(&self, src: &str) -> Option<usize>;

    fn is_end(&self, src: &str) -> Option<usize>;

    fn is_inline(&self, src: &str) -> Option<usize>;

    fn trim<'a>(&self, src: &'a str) -> &'a str;
}

pub enum StandardSpec {
    C,
    Rust,
    HTML,
    Bash,
}

impl Spec for StandardSpec {
    fn is_begin(&self, src: &str) -> Option<usize> {
        match self {
            StandardSpec::C => c_is_begin(src),
            StandardSpec::HTML => html_is_begin(src),
            StandardSpec::Rust => c_is_inline(src),
            _ => None,
        }
    }

    fn is_end(&self, src: &str) -> Option<usize> {
        match self {
            StandardSpec::C => c_is_end(src),
            StandardSpec::HTML => html_is_end(src),
            _ => None,
        }
    }

    fn is_inline(&self, src: &str) -> Option<usize> {
        match self {
            StandardSpec::C => c_is_inline(src),
            StandardSpec::Rust => rust_is_inline(src),
            StandardSpec::Bash => bash_is_inline(src),
            _ => None
        }
    }

    fn trim<'a>(&self, src: &'a str) -> &'a str {
        match self {
            StandardSpec::C => c_trim(src),
            StandardSpec::Rust => rust_trim(src),
            StandardSpec::Bash => bash_trim(src),
            _ => src,
        }
    }
}

fn c_is_begin(src: &str) -> Option<usize> {
    if src.starts_with("/**") {
        Some(3)
    } else if src.starts_with("/*") {
        Some(2)
    } else {
        None
    }
}

fn c_is_end(src: &str) -> Option<usize> {
    if src.starts_with("*/") {
        Some(2)
    } else {
        None
    }
}

fn c_is_inline(src: &str) -> Option<usize> {
    if src.starts_with("//") {
        Some(2)
    } else {
        None
    }
}

fn c_trim<'a>(src: &'a str) -> &'a str {
    if src.starts_with("*") {
        src[1..].as_ref()
    } else if src.starts_with("//") {
        src[2..].as_ref()
    } else {
        src
    }
}

fn rust_is_inline(src: &str) -> Option<usize> {
    if src.starts_with("///") || src.starts_with("//!") {
        Some(3)
    } else {
        c_is_inline(src)
    }
}

fn rust_trim<'a>(src: &'a str) -> &'a str {
    if src.starts_with("///") || src.starts_with("//!") {
        src[3..].as_ref()
    } else {
        c_trim(src)
    }
}

fn html_is_begin(src: &str) -> Option<usize> {
    if src.starts_with("<!--") {
        Some(4)
    } else {
        None
    }
}

fn html_is_end(src: &str) -> Option<usize> {
    if src.starts_with("-->") {
        Some(3)
    } else {
        None
    }
}

fn bash_is_inline(src: &str) -> Option<usize> {
    if src.starts_with("#") {
        Some(1)
    } else {
        None
    }
}

fn bash_trim<'a>(src: &'a str) -> &'a str {
    if src.starts_with("#") {
        src[1..].as_ref()
    } else {
        src
    }
}
