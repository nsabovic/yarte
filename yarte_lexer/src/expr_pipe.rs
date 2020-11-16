use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Result, Token,
};

pub(super) struct ExprPipe {
    list: Punctuated<Expr, Token![=>]>,
}

impl Parse for ExprPipe {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut list = Punctuated::new();
        if !input.is_empty() {
            loop {
                let value = input.parse()?;
                list.push_value(value);
                if !input.peek(Token![=>]) {
                    break;
                }
                let punc = input.parse()?;
                list.push_punct(punc);
            }
        }
        Ok(ExprPipe { list })
    }
}

impl Into<Vec<crate::Expr>> for ExprPipe {
    fn into(self) -> Vec<crate::Expr> {
        self.list
            .into_pairs()
            .map(|p| crate::Expr(p.into_value()))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use syn::parse_str;

    use super::*;

    #[test]
    fn test() {
        let src = "bar => foo = \"bar => \"\n => fuu = 1  => goo = true    ";
        let expected = vec![
            parse_str::<crate::Expr>("bar").unwrap(),
            parse_str::<crate::Expr>("foo=\"bar => \"").unwrap(),
            parse_str::<crate::Expr>("fuu=1").unwrap(),
            parse_str::<crate::Expr>("goo=true").unwrap(),
        ];

        let res: Vec<crate::Expr> = parse_str::<ExprPipe>(src).unwrap().into();

        assert_eq!(expected, res);

        let src = "bar => foo = \"bar => \"\n => fuu = 1  => goo = true   => ";
        assert!(parse_str::<ExprPipe>(src).is_err());

        let src = "                 \n\t ";
        assert!(parse_str::<ExprPipe>(src).is_ok());
    }
}
