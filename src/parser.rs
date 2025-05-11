use peg;

#[derive(Debug, PartialEq)]
pub enum UnoType {
    U32,
    U64,
}

#[derive(Debug, PartialEq)]
pub struct Literal(String);

#[derive(Debug, PartialEq)]
pub enum Expression {
    Func(Func),
    Ret(Vec<Literal>),
}

#[derive(Debug, PartialEq)]
pub struct Module {
    pub expressions: Vec<Expression>
}

#[derive(Debug, PartialEq)]
pub struct Identifier(String);

#[derive(Debug, PartialEq)]
pub struct Func {
    pub name: Identifier,
    pub arg_types: Vec<UnoType>,
    pub arguments: Vec<Identifier>,
    pub return_types: Vec<UnoType>,
    pub expressions: Vec<Expression>,
}

peg::parser! {
    pub grammar uno_parser() for str {
       pub rule module() -> Module
           = ee:expressions() {Module {expressions: ee}}

       pub rule func() -> Expression
           = [' '| '\t' | '\n']* "fn" _ name:id()
            "(" arg_types:args() ")" _ return_types:types() _ "{" "\n"*
            ee:expressions()
            _ "}" "\n"*
            {
                let mut arguments = Vec::with_capacity(arg_types.len());
                let mut arg_tys = Vec::with_capacity(arg_types.len());
                for (arg, type_) in arg_types {
                    arguments.push(arg);
                    arg_tys.push(type_);
                }
                Expression::Func( Func { name, arg_types: arg_tys, arguments, return_types, expressions: ee } )
            }

       pub rule id() -> Identifier
           = ident:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { Identifier(ident.to_owned()) }

       pub rule args() -> Vec<(Identifier, UnoType)>
           = args:(arg()) ** (",") { args }

       pub rule arg() -> (Identifier, UnoType)
           = _ ident:id() _ ty:type_() _ {(ident, ty)}

       pub rule type_() -> UnoType
           = "u32" { UnoType::U32 }
            /"u64" { UnoType::U64 }

       pub rule types() -> Vec<UnoType>
           = tys:(type_()) ** ("," _) {tys}

       pub rule expressions() -> Vec<Expression>
           = ii:(expression())* { ii }

       pub rule expression() -> Expression
           = ret()
           / func()

       pub rule ret() -> Expression
           = _ "return" _ ls:literals() "\n" { Expression::Ret(ls) }

       pub rule literals() -> Vec<Literal>
           = ls:(literal()) ** ("," _) { ls }

       pub rule literal() -> Literal
           = v:$(['0'..='9']*) { Literal(v.to_owned()) }

       pub rule unknown_instr() -> Expression
           = expected!("Unknown instruction")

       rule _() =  quiet!{[' ' | '\t']*}
    }
}

#[cfg(test)]
mod test {
    use crate::parser::*;

    #[test]
    fn test_func_parser() {
        let code = r#"fn h8(class u32, text u32) u32 {
            return 0
        }"#;

        assert_eq!(
            uno_parser::func(code).expect("should parse without errors"),
            Expression::Func(Func {
                name: Identifier("h8".to_owned()),
                arg_types: vec![UnoType::U32, UnoType::U32],
                arguments: vec![Identifier("class".into()), Identifier("text".into())],
                return_types: vec![UnoType::U32],
                expressions: vec![Expression::Ret(vec![Literal("0".to_owned())])]
            })
        );
    }
}
