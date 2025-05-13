use peg;

#[derive(Debug, PartialEq)]
pub enum UnoType {
    U32,
}

#[derive(Debug, PartialEq)]
pub enum Val {
    U32(u32),
}

#[derive(Debug, PartialEq)]
pub enum Instr {
    Ret(Vec<Val>),
}

#[derive(Debug, PartialEq)]
pub struct Identifier(String);

#[derive(Debug, PartialEq)]
pub struct Func {
    pub name: Identifier,
    pub arg_types: Vec<UnoType>,
    pub arguments: Vec<Identifier>,
    pub return_types: Vec<UnoType>,
    pub insrs: Vec<Instr>,
}

#[derive(Debug, PartialEq)]
pub enum AST {
    Func(Func),
}

peg::parser! {
    pub grammar uno_parser() for str {
       pub rule func() -> Func
           = [' '| '\t' | '\n']* "fn" _ name:id()
            "(" arg_types:args() ")" _ return_types:types() _ "{\n"
            _ "\n"*
            _ "}" "\n"*
            {
                let mut arguments = Vec::with_capacity(arg_types.len());
                let mut arg_tys = Vec::with_capacity(arg_types.len());
                for (arg, type_) in arg_types {
                    arguments.push(arg);
                    arg_tys.push(type_);
                }
                Func { name, arg_types: arg_tys, arguments, return_types, insrs: vec![] } }

       pub rule id() -> Identifier
           = ident:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { Identifier(ident.to_owned()) }

       pub rule args() -> Vec<(Identifier, UnoType)>
           = args:(arg()) ** (",") { args }

       pub rule arg() -> (Identifier, UnoType)
           = _ ident:id() _ ty:type_() _ {(ident, ty)}

       pub rule type_() -> UnoType
           = "u32" { UnoType::U32 }

       pub rule types() -> Vec<UnoType>
           = tys:(type_()) ** ("," _) {tys}

       rule _() =  quiet!{[' ' | '\t']*}
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{uno_parser, Func, Identifier, UnoType};

    #[test]
    fn test_newline() {
        let code = r#"fn h8(class u32, text u32) u32 {
        }"#;

        assert_eq!(
            uno_parser::func(code).expect("should parse without errors"),
            Func {
                name: Identifier("h8".to_owned()),
                arg_types: vec![UnoType::U32, UnoType::U32],
                arguments: vec![Identifier("class".into()), Identifier("text".into())],
                return_types: vec![UnoType::U32],
                insrs: vec![]
            }
        );
    }
}
