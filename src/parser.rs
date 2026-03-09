use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Expr, Ident, Pat, Path, Result, Token,
};

use crate::attribute::Attribute;
use crate::node::{Content, ForBlock, IfBlock, Markup, Node};

impl Parse for Markup {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            root: input.parse()?,
        })
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Path = input.parse()?;

        let args = if input.peek(token::Paren) {
            let content;
            let _ = syn::parenthesized!(content in input);
            content.parse_terminated(Expr::parse, Token![,])?
        } else {
            Punctuated::new()
        };

        let attributes = if input.peek(Token![!]) && input.peek2(token::Bracket) {
            let _ = input.parse::<Token![!]>()?;
            let content;
            let _ = syn::bracketed!(content in input);
            let parsed_attrs = content.parse_terminated(Attribute::parse, Token![,])?;
            parsed_attrs.into_iter().collect()
        } else {
            Vec::new()
        };

        let style = if input.peek(Token![|]) {
            let _ = input.parse::<Token![|]>()?;
            Some(input.parse::<Expr>()?)
        } else {
            None
        };

        let children = if input.peek(token::Brace) {
            let content;
            let _ = syn::braced!(content in input);
            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse::<Content>()?);
                if content.peek(Token![,]) {
                    let _ = content.parse::<Token![,]>()?;
                }
            }
            items
        } else {
            Vec::new()
        };

        Ok(Self {
            name,
            args,
            attributes,
            style,
            children,
        })
    }
}

impl Parse for Content {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![if]) {
            Ok(Self::If(input.parse()?))
        } else if input.peek(Token![for]) {
            Ok(Self::For(input.parse()?))
        } else if input.peek(Token![@]) {
            let _ = input.parse::<Token![@]>()?;
            let name: Ident = input.parse()?;
            let _: Token![:] = input.parse()?;
            let content: Self = input.parse()?;
            Ok(Self::Slot(name, Box::new(content)))
        } else {
            // Robust check: Is this a Path followed by widget tokens?
            let is_widget = {
                let fork = input.fork();
                if fork.parse::<Path>().is_ok() {
                    fork.peek(token::Paren)
                        || fork.peek(token::Brace)
                        || fork.peek(Token![!])
                        || fork.peek(Token![|])
                } else {
                    false
                }
            };

            if is_widget {
                Ok(Self::Widget(input.parse()?))
            } else {
                Ok(Self::Expr(input.parse()?))
            }
        }
    }
}

impl Parse for IfBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let _ = input.parse::<Token![if]>()?;

        // Use a fork to parse the condition up to the brace
        let condition = parse_expr_until_brace(input)?;

        let then_content;
        let _ = syn::braced!(then_content in input);
        let mut then_branch = Vec::new();
        while !then_content.is_empty() {
            then_branch.push(then_content.parse()?);
            if then_content.peek(Token![,]) {
                let _ = then_content.parse::<Token![,]>()?;
            }
        }

        let else_branch = if input.peek(Token![else]) {
            let _ = input.parse::<Token![else]>()?;
            let else_content;
            let _ = syn::braced!(else_content in input);
            let mut items = Vec::new();
            while !else_content.is_empty() {
                items.push(else_content.parse()?);
                if else_content.peek(Token![,]) {
                    let _ = else_content.parse::<Token![,]>()?;
                }
            }
            Some(items)
        } else {
            None
        };

        Ok(Self {
            condition,
            then_branch,
            else_branch,
        })
    }
}

impl Parse for ForBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let _ = input.parse::<Token![for]>()?;
        let pat: Pat = Pat::parse_single(input)?;
        let _ = input.parse::<Token![in]>()?;

        let expr = parse_expr_until_brace(input)?;

        let body_content;
        let _ = syn::braced!(body_content in input);
        let mut body = Vec::new();
        while !body_content.is_empty() {
            body.push(body_content.parse()?);
            if body_content.peek(Token![,]) {
                let _ = body_content.parse::<Token![,]>()?;
            }
        }

        Ok(Self { pat, expr, body })
    }
}

/// Helper to parse an expression but stop BEFORE an opening brace.
/// This prevents 'if cond { ... }' from being parsed as a struct literal.
fn parse_expr_until_brace(input: ParseStream) -> Result<Expr> {
    let mut tokens = proc_macro2::TokenStream::new();
    while !input.is_empty() && !input.peek(token::Brace) {
        tokens.extend(std::iter::once(input.parse::<proc_macro2::TokenTree>()?));
    }
    syn::parse2(tokens)
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let is_event = if input.peek(Token![+]) {
            let _ = input.parse::<Token![+]>()?;
            true
        } else {
            false
        };

        let name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let value: Expr = input.parse()?;

        Ok(Self {
            name,
            is_event,
            value,
        })
    }
}
