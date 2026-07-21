use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

fn path<'a>(segments: &'a [&'a str]) -> impl Iterator<Item = TokenTree> + 'a {
    segments.iter().enumerate().flat_map(move |(i, seg)| {
        let mut v = Vec::new();
        if i > 0 {
            v.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
            v.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        }
        v.push(TokenTree::Ident(Ident::new(seg, Span::call_site())));
        v
    })
}

fn macro_invocation(
    path_tokens: impl IntoIterator<Item = TokenTree>,
    body: TokenStream,
) -> TokenStream {
    let mut out = TokenStream::new();
    out.extend(path_tokens);
    out.extend([TokenTree::Punct(Punct::new('!', Spacing::Alone))]);
    out.extend([TokenTree::Group(Group::new(Delimiter::Brace, body))]);
    out
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
        let mut paren_inner = paren.stream().into_iter();
        let Some(TokenTree::Ident(c)) = paren_inner.next() else {
            continue;
        };
        if ["C", "transparent"].contains(&c.to_string().as_str()) {
            return true;
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
        path(&["const_shader_layout", "shader_layout_assert"]),
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
    let mut out = macro_invocation(
        path(&["const_shader_layout", "shader_layout_assert"]),
        input.clone(),
    );
    out.extend(macro_invocation(
        path(&["const_shader_layout", "shader_layout_compat_assert"]),
        input,
    ));
    out
}
