use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

fn path<'a>(segments: &'a [&'a str]) -> TokenStream {
    let mut out = TokenStream::new();
    for (i, seg) in segments.iter().enumerate() {
        if i > 0 {
            out.extend([
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            ]);
        }
        out.extend([TokenTree::Ident(Ident::new(seg, Span::call_site()))]);
    }
    out
}

fn macro_invocation(mut path_tokens: TokenStream, body: TokenStream) -> TokenStream {
    path_tokens.extend([TokenTree::Punct(Punct::new('!', Spacing::Alone))]);
    path_tokens.extend([TokenTree::Group(Group::new(Delimiter::Brace, body))]);
    path_tokens
}

fn compile_error(msg: &str) -> TokenStream {
    format!("compile_error!({msg:?});").parse().unwrap()
}

fn has_repr_c_or_transparent(input: &TokenStream) -> bool {
    let mut tokens = input.clone().into_iter().peekable();
    while let Some(token) = tokens.next() {
        let TokenTree::Punct(p) = token else { continue };
        if p.as_char() != '#' {
            continue;
        }
        let Some(TokenTree::Group(g)) = tokens.next() else {
            continue;
        };
        if g.delimiter() != Delimiter::Bracket {
            continue;
        }
        let mut inner = g.stream().into_iter();
        let Some(TokenTree::Ident(first)) = inner.next() else {
            continue;
        };
        if first.to_string() != "repr" {
            continue;
        }
        let Some(TokenTree::Group(paren)) = inner.next() else {
            continue;
        };
        if paren.delimiter() != Delimiter::Parenthesis {
            continue;
        }
        for token in paren.stream() {
            if let TokenTree::Ident(c) = token
                && ["C", "transparent"].contains(&c.to_string().as_str())
            {
                return true;
            }
        }
    }
    false
}

#[proc_macro_derive(ShaderLayout)]
pub fn derive_shader_layout(input: TokenStream) -> TokenStream {
    if !has_repr_c_or_transparent(&input) {
        return compile_error(
            "Struct must be `#[repr(C)]` or `#[repr(transparent)]` for `ShaderLayout`",
        );
    }
    macro_invocation(
        path(&["const_shader_layout", "impl_shader_layout_struct"]),
        input,
    )
}

#[proc_macro_derive(ShaderLayoutCompat)]
pub fn derive_shader_layout_compat(input: TokenStream) -> TokenStream {
    if !has_repr_c_or_transparent(&input) {
        return compile_error(
            "Struct must be `#[repr(C)]` or `#[repr(transparent)]` for `ShaderLayoutCompat`",
        );
    }
    macro_invocation(
        path(&["const_shader_layout", "impl_shader_layout_compat_struct"]),
        input,
    )
}

/// Returns the identifier of the struct definition in the input,
/// i.e. the identifier right after the `struct` keyword.
fn struct_ident(input: &TokenStream) -> Option<Ident> {
    let mut tokens = input.clone().into_iter();
    while let Some(token) = tokens.next() {
        let TokenTree::Ident(ident) = token else {
            continue;
        };
        if ident.to_string() != "struct" {
            continue;
        }
        return match tokens.next() {
            Some(TokenTree::Ident(struct_name)) => Some(struct_name),
            _ => None,
        };
    }
    None
}

#[proc_macro_derive(ShaderLayoutCompatArrayElement)]
pub fn derive_shader_layout_compat_array_element(input: TokenStream) -> TokenStream {
    // `impl_shader_layout_compat_array_element!` expects type identifiers,
    // not the whole struct definition, so extract the struct's identifier.
    let Some(struct_name) = struct_ident(&input) else {
        return compile_error("`ShaderLayoutCompatArrayElement` can only be derived for structs");
    };
    macro_invocation(
        path(&[
            "const_shader_layout",
            "impl_shader_layout_compat_array_element",
        ]),
        TokenStream::from(TokenTree::Ident(struct_name)),
    )
}
