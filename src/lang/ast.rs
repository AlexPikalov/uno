/// Uno lang expression.
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// function declaration
    FuncDecl(Func),
    Return(Option<Expression>),
    EmptyLine,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    /// Numerical constant
    NConst(String),
    /// String constant
    StrConst(String),
    /// Identifier
    Ident(String),
    /// Function Call
    FuncCall {
        fn_name: String,
        args: Vec<Expression>,
    },
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
    pub grammar expr_parser() for str {
        // FIXME: make string working with escaped "
        pub rule str_const() -> Expression
            = "\"" s:$([^ '\n' | '"']* )  "\"" { Expression::StrConst(s.to_owned()) }

        pub rule n_const() -> Expression
            = n_const_dec()
            / n_const_hex()
            / n_const_bin()

        pub rule expr() -> Expression
            = str_const()
            / n_const()
            / func_call()
            / ident_expr()

        pub rule n_const_dec() -> Expression
            = !("0" ['x' | 'b']) n:$(['0'..='9' | '_']*) { Expression::NConst(n.to_owned()) }

        pub rule n_const_hex() -> Expression
            = n:$("0x" ['0'..='9' | 'a'..='f' | 'A'..='F' | '_']*) { Expression::NConst(n.to_owned()) }

        pub rule n_const_bin() -> Expression
            = n:$("0b" ['0' | '1' | '_']*) { Expression::NConst(n.to_owned()) }

        pub rule ident() -> String
            = i:$(['a'..='z' | 'A'..='Z' | '_' | '$'] ['a'..='z' | 'A'..='Z' | '_' | '$' | '0'..='9']*) { i.to_owned() }

        pub rule ident_expr() -> Expression = id:ident() {Expression::Ident(id)}

        pub rule type_() -> UType
            = "i64" { UType::I64 }

        pub rule func() -> Func
            = _ "fn" _ name:ident() "(" _ aa:fn_args() _ ")" _ rty:type_()? _ "{" "\n"?
            body:statements()
            _ "}\n"
            { Func{ name: name.to_owned(), args: aa, ret: rty.unwrap_or(UType::Nothing), body } }

        rule fn_args() -> Vec<(String, UType)>
            = aa:(fn_arg()) ** (","  _) { aa }

        rule fn_arg() -> (String, UType)
            = name:ident() " " _ ty:type_() { (name, ty) }

        pub rule statements() -> Vec<Statement>
            = stmts:(statement()*)
            {
                let mut res = Vec::with_capacity(stmts.len());
                for s in stmts {
                    if s != Statement::EmptyLine {
                        res.push(s);
                    }
                }
                res
            }

        pub rule func_call() -> Expression
            = name:ident() "(" _ aa:(expr()) ** (_ "," _) _ ")"
            { Expression::FuncCall { fn_name: name, args: aa } }

        rule statement() -> Statement
            = func_decl()
            / ret_decl()
            / empty_line()

        rule maybe_statement() -> Option<Statement>
            = stmt:statement()?
            { stmt }

        rule func_decl() -> Statement = fn_:func() { Statement::FuncDecl(fn_) }

        rule ret_decl() -> Statement = _ "return " _ rexpr:expr()? _ "\n" { Statement::Return(rexpr) }

        rule _() = quiet!{[' ' | '\t']*}

        rule empty_line() -> Statement = _ "\n" { Statement::EmptyLine }
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
            Expression::StrConst("".to_owned())
        );
        assert_eq!(
            expr_parser::str_const("\"hi\"").expect("should parse non-empty string"),
            Expression::StrConst("hi".to_owned())
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
            Expression::NConst("01235".to_owned())
        );
        assert_eq!(
            expr_parser::n_const("1_235_000")
                .expect("should capture decimal number with separator"),
            Expression::NConst("1_235_000".to_owned())
        );
        // hexadecimal
        assert_eq!(
            expr_parser::n_const("0x12_3456_789abc_defAB_CDEF")
                .expect("should capture hexadecimal number"),
            Expression::NConst("0x12_3456_789abc_defAB_CDEF".to_owned())
        );
        // binary
        assert_eq!(
            expr_parser::n_const("0b011_111").expect("should capture binary number"),
            Expression::NConst("0b011_111".to_owned())
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
            expr_parser::func("fn () {}\n").is_err(),
            "should not match a non-func expression"
        );

        assert_eq!(
            expr_parser::func("fn main() {}\n")
                .expect("should match function with empty body and no return type"),
            Func {
                name: "main".to_owned(),
                args: vec![],
                ret: UType::Nothing,
                body: vec![]
            }
        );
        assert_eq!(
            expr_parser::func("fn main()       i64    {\n\n\t\t}\n")
                .expect("should match function with empty body"),
            Func {
                name: "main".to_owned(),
                args: vec![],
                ret: UType::I64,
                body: vec![]
            }
        );
        assert_eq!(
            expr_parser::func("fn main() i64 {\nreturn 12\n}\n")
                .expect("should match function with return"),
            Func {
                name: "main".to_owned(),
                args: vec![],
                ret: UType::I64,
                body: vec![Statement::Return(Some(Expression::NConst("12".to_owned())))]
            }
        );
    }

    #[test]
    fn func_call_test() {
        assert_eq!(
            expr_parser::func_call("hello(1, \"world\", some_id)").expect("should match function call"),
            Expression::FuncCall {
                fn_name: "hello".to_owned(),
                args: vec![
                    Expression::NConst("1".to_owned()),
                    Expression::StrConst("world".to_owned()),
                    Expression::Ident("some_id".to_owned())
                ]
            }
        );
        assert_eq!(
            expr_parser::expr("hello(1, \"world\")").expect("should match function call expression"),
            Expression::FuncCall {
                fn_name: "hello".to_owned(),
                args: vec![
                    Expression::NConst("1".to_owned()),
                    Expression::StrConst("world".to_owned())
                ]
            }
        );
        assert_eq!(
            expr_parser::func_call("hello(\"world\")").expect("should match function call"),
            Expression::FuncCall {
                fn_name: "hello".to_owned(),
                args: vec![Expression::StrConst("world".to_owned())]
            }
        );
        assert_eq!(
            expr_parser::func_call("return hello(\"world\")\n")
                .expect("should match returned function call"),
            Expression::FuncCall {
                fn_name: "hello".to_owned(),
                args: vec![Expression::StrConst("world".to_owned())]
            }
        );
    }
}
