//! # commentator
//! 
//! `commentator` is a crate which provies binary and SDK
//! for extracting comment entries from source code. SDK provides
//! API such as `commentator::Tokenizer` to push source code line by
//! line into state machine and then pull extracted comments entries
//! with some context, such as line number and offset position.
//!
//! Binary could be used as CLI tool to extract comments from source
//! code file and print it to stdout.
// pub use tokenizer::Tokenizer as Tokenizer;
mod tokenizer;
pub use tokenizer::{Tokenizer, Comment};
pub mod spec;

#[cfg(test)]
mod test {
    use crate::tokenizer::Tokenizer;
    use crate::spec;
    use crate::spec::Spec;

    #[test]
    fn tokenizer_test() {
        let mut t = Tokenizer::new(spec::Java::new());
        t.update(1, "/*\n");
        t.update(2, " * Entry point.\n");
        t.update(3, " */\n");
        t.update(4, "public static void main(String... args) {\n");
        t.update(5, "  System.out.println(\"hello world\");\n");
        t.update(6, "}\n");
        let cmt = t.take();
        assert!(cmt.is_some());
        assert_eq!(cmt.unwrap().text, "\n * Entry point.\n ");
        assert!(t.take().is_none());
    }

    #[test]
    fn spec_java_detect_comment() {
        let s = &spec::Java::new();
        test_begin(s, "/* comment", 2);
        test_begin(s, "/** comment", 3);
        test_end(s, "*/", 2);
        test_inline(s, "// comment", 2);
    }

    #[test]
    fn spec_html_detect_comments() {
        let s = &spec::HTML::new();
        test_begin(s, "<!-- comment", 4);
        test_end(s, "-->", 3);
    }

    #[test]
    fn spec_bash_detect_comment() {
        let s = &spec::Bash::new();
        test_inline(s, "# comment", 1);
    }

    fn test_begin<S: Spec>(spec: &S, src: &str, offset: usize) {
        if let Some(o) = spec.is_begin(src) {
            assert_eq!(o, offset, "incorrect begin offset");
        } else {
            assert!(false, "begin comment was not found");
        }
    }

    fn test_end<S: Spec>(spec: &S, src: &str, offset: usize) {
        if let Some(o) = spec.is_end(src) {
            assert_eq!(o, offset, "incorrect end offset");
        } else {
            assert!(false, "end comment was not found");
        }
    }

    fn test_inline<S: Spec>(spec: &S, src: &str, offset: usize) {
        if let Some(o) = spec.is_inline(src) {
            assert_eq!(o, offset, "incorrect inline offset");
        } else {
            assert!(false, "inline comment was not found");
        }
    }
}
