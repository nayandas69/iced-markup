use crate::node::{Content, ForBlock, IfBlock, Markup, Node};
use quote::{quote, ToTokens};

impl ToTokens for Markup {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        self.root.to_tokens(tokens);
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let name = &self.name;
        let args = &self.args;

        // Determine the fully qualified name or helper
        let mut widget_path = quote! { #name };
        let mut is_layout = false;

        if name.segments.len() == 1 {
            let ident = name.segments[0].ident.to_string();
            match ident.as_str() {
                "column" | "row" | "text" | "button" | "container" | "scrollable" => {
                    widget_path = quote! { iced::widget::#name };
                    if ident == "column" || ident == "row" {
                        is_layout = true;
                    }
                }
                _ => {}
            }
        }

        // Base widget construction
        let mut widget_tokens = if is_layout && !self.children.is_empty() {
            quote! { #widget_path([]) }
        } else {
            quote! { #widget_path(#args) }
        };

        // Apply style pipe if present
        if let Some(style) = &self.style {
            widget_tokens.extend(quote! { .style(#style) });
        }

        // Apply attributes
        for attr in &self.attributes {
            let name = &attr.name;
            let value = &attr.value;

            if attr.is_event {
                let method = match name.to_string().as_str() {
                    "click" => quote! { on_press },
                    "input" => quote! { on_input },
                    "submit" => quote! { on_submit },
                    _ => quote! { #name },
                };
                widget_tokens.extend(quote! { .#method(#value) });
            } else {
                widget_tokens.extend(quote! { .#name(#value) });
            }
        }

        // Handle children
        if self.children.is_empty() {
            tokens.extend(widget_tokens);
        } else {
            let has_control_flow = self
                .children
                .iter()
                .any(|c| matches!(c, Content::If(_) | Content::For(_)));

            if has_control_flow {
                let children_codegen = self.children.iter().filter_map(|c| {
                    match c {
                        Content::Widget(node) => Some(quote! { __children.push(#node.into()); }),
                        Content::If(if_block) => Some(quote! { #if_block }),
                        Content::For(for_block) => Some(quote! { #for_block }),
                        Content::Expr(expr) => Some(quote! { __children.push(#expr.into()); }),
                        Content::Slot(_, _) => None, // Slots are applied separately
                    }
                });

                let slots_codegen = self.children.iter().filter_map(|c| {
                    if let Content::Slot(name, content) = c {
                        Some(quote! { __widget = __widget.#name(#content); })
                    } else {
                        None
                    }
                });

                tokens.extend(quote! {
                    {
                        let mut __children = Vec::<iced::Element<'_, _, _, _>>::new();
                        #(#children_codegen)*
                        let mut __widget = #widget_tokens;
                        for __child in __children {
                            __widget = __widget.push(__child);
                        }
                        #(#slots_codegen)*
                        __widget
                    }
                });
            } else {
                // Static optimization
                for child in &self.children {
                    match child {
                        Content::Widget(node) => {
                            widget_tokens.extend(quote! { .push(#node) });
                        }
                        Content::Slot(name, content) => {
                            widget_tokens.extend(quote! { .#name(#content) });
                        }
                        Content::Expr(expr) => {
                            widget_tokens.extend(quote! { .push(#expr) });
                        }
                        _ => unreachable!("Static optimization shouldn't have control flow"),
                    }
                }
                tokens.extend(widget_tokens);
            }
        }
    }
}

impl ToTokens for Content {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            Self::Widget(node) => node.to_tokens(tokens),
            Self::Slot(name, content) => {
                tokens.extend(quote! { .#name(#content) });
            }
            Self::If(if_block) => if_block.to_tokens(tokens),
            Self::For(for_block) => for_block.to_tokens(tokens),
            Self::Expr(expr) => expr.to_tokens(tokens),
        }
    }
}

impl ToTokens for IfBlock {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let condition = &self.condition;
        let then_branch = self
            .then_branch
            .iter()
            .map(|c| quote! { __children.push(#c.into()); });

        let else_tokens = self.else_branch.as_ref().map_or_else(
            || quote! {},
            |else_branch| {
                let else_content = else_branch
                    .iter()
                    .map(|c| quote! { __children.push(#c.into()); });
                quote! { else { #(#else_content)* } }
            },
        );

        tokens.extend(quote! {
            if #condition {
                #(#then_branch)*
            } #else_tokens
        });
    }
}

impl ToTokens for ForBlock {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let pat = &self.pat;
        let expr = &self.expr;
        let body = self
            .body
            .iter()
            .map(|c| quote! { __children.push(#c.into()); });

        tokens.extend(quote! {
            for #pat in #expr {
                #(#body)*
            }
        });
    }
}
