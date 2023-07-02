use std::{ffi::CString, mem::MaybeUninit, path::Path};

use yarp_sys::{yp_constant_t, yp_parse, yp_parser_init, yp_parser_t};

use crate::{
    ast::Ast, comment::Comment, diagnostic::Diagnostic, encoding::Encoding, list::ListRef,
};

pub struct Parser {
    inner: yp_parser_t,

    // We need to take ownership of the source string to keep it around for the lifetime of
    // the parser.
    _source: CString,

    // Same with this, if it was given.
    _file_path: Option<CString>,
}

impl Parser {
    pub fn try_new(source: &str, file_path: Option<&Path>) -> Result<Self, std::ffi::NulError> {
        let mut parser = MaybeUninit::<yp_parser_t>::uninit();
        let c_source = CString::new(source)?;

        let c_file_path = file_path.map(|fp| {
            let s = fp.to_str().expect("FIXME");
            CString::new(s).expect("FIXME")
        });

        let file_path_ptr = match c_file_path.as_ref() {
            Some(c_string) => c_string.as_ptr(),
            None => std::ptr::null(),
        };
        let inner = unsafe {
            yp_parser_init(
                parser.as_mut_ptr(),
                c_source.as_ptr(),
                c_source.as_bytes().len(),
                file_path_ptr,
            );

            if parser.as_ptr().is_null() {
                panic!("asdf");
            }

            parser.assume_init()
        };

        Ok(Self {
            inner,
            _source: c_source,
            _file_path: c_file_path,
        })
    }

    pub fn parse(&mut self) -> ParseResult {
        let node = unsafe { yp_parse(&mut self.inner) };
        assert!(!node.is_null(), "Parse result was null");

        let ast = Ast::try_new(self, node).expect("Node wasn't a Program?");

        ParseResult {
            ast,
            comments: self.comments(),
            errors: self.errors(),
            warnings: self.warnings(),
        }
    }

    pub(crate) fn inner_mut(&mut self) -> &mut yp_parser_t {
        &mut self.inner
    }

    pub(crate) fn encoding(&self) -> Encoding<'_> {
        Encoding::new(&self.inner.encoding)
    }

    fn comments(&self) -> Vec<Comment> {
        let c_comment_list = &self.inner.comment_list;
        let comment_list = ListRef::new(c_comment_list);

        comment_list
            .iter()
            .map(|list_node| {
                let ptr = list_node.to_comment_ptr();
                Comment::inner_new(ptr, &self.inner)
            })
            .collect()
    }

    fn errors(&self) -> Vec<Diagnostic> {
        let c_error_list = &self.inner.error_list;
        let error_list = ListRef::new(c_error_list);

        error_list
            .iter()
            .map(|list_node| {
                let ptr = list_node.to_diagnostic_ptr();
                Diagnostic::new(ptr, &self.inner)
            })
            .collect()
    }

    fn warnings(&self) -> Vec<Diagnostic> {
        let c_warning_list = &self.inner.warning_list;
        let warning_list = ListRef::new(c_warning_list);

        warning_list
            .iter()
            .map(|list_node| {
                let ptr = list_node.to_diagnostic_ptr();
                Diagnostic::new(ptr, &self.inner)
            })
            .collect()
    }

    pub(crate) fn start(&self) -> Option<usize> {
        unsafe { self.inner.start.as_ref().map(|v| *v as usize) }
    }

    pub(crate) fn constant_pool(&self) -> &[yp_constant_t] {
        unsafe {
            let constants = self.inner.constant_pool.constants;

            if constants.is_null() {
                return &[];
            }

            std::slice::from_raw_parts(constants, self.inner.constant_pool.size)
        }
    }
}

pub struct ParseResult<'a> {
    ast: Ast<'a>,
    comments: Vec<Comment>,
    errors: Vec<Diagnostic>,
    warnings: Vec<Diagnostic>,
}

impl<'a> ParseResult<'a> {
    pub fn comments(&self) -> &[Comment] {
        self.comments.as_ref()
    }

    pub fn errors(&self) -> &[Diagnostic] {
        self.errors.as_ref()
    }

    pub fn warnings(&self) -> &[Diagnostic] {
        self.warnings.as_ref()
    }

    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

#[cfg(test)]
mod tests {
    use crate::comment::CommentType;

    use super::*;

    fn ruby_file_contents() -> String {
        // let rust_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        // let ruby_file_path = rust_path.join("../lib/yarp.rb").canonicalize().unwrap();
        // std::fs::read_to_string(ruby_file_path).unwrap()
        "# Blarg
class Foo; end"
            .to_string()
    }

    #[test]
    fn try_new_test() {
        let _parser =
            Parser::try_new(&ruby_file_contents(), None).expect("This should not segfault");
    }

    #[test]
    fn parse_test() {
        let code = r#"# A comment, some errors, and a warning.
        class Foo
        puts /x/
        "#;
        let mut parser = Parser::try_new(code, None).unwrap();
        let result = parser.parse();

        // Comments
        {
            let comments = result.comments();

            assert_eq!(comments.len(), 1);
            assert_eq!(comments[0].location().as_range(), &(0usize..41usize));
            assert_eq!(comments[0].type_(), CommentType::Inline);
        }

        // Errors
        {
            let errors = result.errors();
            assert_eq!(errors.len(), 2);

            // assert_eq!(errors[0].location(), &(17usize..17usize));
            assert_eq!(
                errors[0].message(),
                "Expected to be able to parse an expression."
            );

            // assert_eq!(errors[1].location(), &(17usize..17usize));
            assert_eq!(
                errors[1].message(),
                "Expected `end` to close `class` statement."
            );
        }

        // Warnings
        {
            let warnings = result.warnings();
            assert_eq!(warnings.len(), 1);

            assert_eq!(warnings[0].location(), &(72usize..73usize));
            assert_eq!(
                warnings[0].message(),
                "ambiguity between regexp and two divisions: wrap regexp in parentheses or add a space after `/' operator"
            );
        }
    }
}
