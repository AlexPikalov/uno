/// Uno lang expression.
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// function declaration
    FuncDecl(Func),
    Return(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    /// Numerical constant
    NConst(String),
    /// String constant
    StrConst(String),
}

#[derive(Debug, PartialEq)]
pub enum UType {
    I64,
    Nothing,
}

#[derive(Debug, PartialEq)]
pub struct Func {
    pub name: String,
    pub args: Vec<(String, UType)>,
    pub ret: UType,
    pub body: Vec<Statement>,
}

peg::parser! {
    grammar expr_parser() for str {
        // FIXME: make string working with escaped "
        pub rule str_const() -> String
            = "\"" s:$([^ '\n' | '"']* )  "\"" { s.to_owned() }

        pub rule n_const() -> String
            = n_const_dec()
            / n_const_hex()
            / n_const_bin()

        pub rule n_const_dec() -> String
            = !("0" ['x' | 'b']) n:$(['0'..='9' | '_']*) { n.to_owned() }

        pub rule n_const_hex() -> String
            = n:$("0x" ['0'..='9' | 'a'..='f' | 'A'..='F' | '_']*) { n.to_owned() }

        pub rule n_const_bin() -> String
            = n:$("0b" ['0' | '1' | '_']*) { n.to_owned() }

        pub rule ident() -> String
            = i:$(['a'..='z' | 'A'..='Z' | '_' | '$'] ['a'..='z' | 'A'..='Z' | '_' | '$' | '0'..='9']*) { i.to_owned() }

        pub rule type_() -> UType
            = "i64" { UType::I64 }

        pub rule func() -> Func
            = _ "fn" _ name:ident() "(" aa:fn_args() ")" _ rty:type_()? _ "{" "\n"?
            (_ "\n")*
            _ "}"
            { Func{ name: name.to_owned(), args: aa, ret: rty.unwrap_or(UType::Nothing), body: vec![] } }

        rule fn_args() -> Vec<(String, UType)>
            = aa:(fn_arg()) ** ("," _) { aa }

        rule fn_arg() -> (String, UType)
            = name:ident() "," _ ty:type_() { (name, ty) }

        rule statements() -> Vec<Statement>
            = stmts:(statement()*)
            { stmts }

        rule _() = quiet!{[' ' | '\t']*}
    }
}

#[cfg(test)]
mod test {
    use crate::lang::ast::{Expression, Func, Statement, UType};

    use super::expr_parser;

    #[test]
    fn str_const_test() {
        assert_eq!(
            expr_parser::str_const("\"\"").expect("should parse empty string"),
            "".to_owned()
        );
        assert_eq!(
            expr_parser::str_const("\"hi\"").expect("should parse non-empty string"),
            "hi".to_owned()
        );
        // FIXME:
        //        assert_eq!(
        //            expr_parser::str_const("\"\\\"hi\\\"\"").expect("should parse string escaped quotes"),
        //            "\"hi\"".to_owned()
        //        );
    }

    #[test]
    fn n_const_test() {
        assert!(
            expr_parser::n_const("hello").is_err(),
            "should not match if input is not a numerical constant"
        );
        // decimal
        assert_eq!(
            expr_parser::n_const("01235").expect("should capture decimal number"),
            "01235"
        );
        assert_eq!(
            expr_parser::n_const("1_235_000")
                .expect("should capture decimal number with separator"),
            "1_235_000"
        );
        // hexadecimal
        assert_eq!(
            expr_parser::n_const("0x12_3456_789abc_defAB_CDEF")
                .expect("should capture hexadecimal number"),
            "0x12_3456_789abc_defAB_CDEF"
        );
        // binary
        assert_eq!(
            expr_parser::n_const("0b011_111").expect("should capture binary number"),
            "0b011_111"
        );
        assert!(
            expr_parser::n_const("0b0121_111").is_err(),
            "should not match malformed binary number"
        );
    }

    #[test]
    fn ident_test() {
        assert!(
            expr_parser::ident("1abd").is_err(),
            "should not match non-ident string"
        );
        assert_eq!(
            expr_parser::ident("$askd12").expect("should match ident"),
            "$askd12"
        );
    }

    #[test]
    fn type_test() {
        assert!(
            expr_parser::type_("i1").is_err(),
            "should not match non-type content"
        );
        assert_eq!(
            expr_parser::type_("i64").expect("should match i64 type"),
            UType::I64
        );
    }

    #[test]
    fn func_test() {
        assert!(
            expr_parser::func("fn () {}").is_err(),
            "should not match a non-func expression"
        );

        assert_eq!(
            expr_parser::func("fn main() {}").expect("should match function"),
            Func {
                name: "main".to_owned(),
                args: vec![],
                ret: UType::Nothing,
                body: vec![]
            }
        );
        assert_eq!(
            expr_parser::func("fn main()       i64    {\n\n\t\t}").expect("should match function"),
            Func {
                name: "main".to_owned(),
                args: vec![],
                ret: UType::I64,
                body: vec![]
            }
        );
        assert_eq!(
            expr_parser::func("fn main() i64 {\nreturn 12\n").expect("should match function"),
            Func {
                name: "main".to_owned(),
                args: vec![],
                ret: UType::I64,
                body: vec![Statement::Return(Expression::NConst("12".to_owned()))]
            }
        );
    }
}
