use proc_macro2::{
    token_stream::{IntoIter, TokenStream},
    TokenTree::{self, *},
};

/// A flattening iterator for the `TokenStream`.
///
/// Its purpose is to essentially iterate at once over all `Ident`s appearing in the input.
/// Note that the punctuation, such as the group delimiters, might be dropped from output.
pub struct TokenStreamFlatten {
    data: Vec<IntoIter>,
}

impl Iterator for TokenStreamFlatten {
    type Item = TokenTree;
    fn next(&mut self) -> Option<TokenTree> {
        let (last, ret) = self.next_recur()?;
        self.data.push(last);
        Some(ret)
    }
}

impl TokenStreamFlatten {
    fn next_recur(&mut self) -> Option<(IntoIter, TokenTree)> {
        let mut last = self.data.pop()?;
        let ret = match last.next() {
            None => {
                let ret_recur = self.next_recur()?;
                last = ret_recur.0;
                ret_recur.1
            }
            Some(Group(group)) => {
                self.data.push(last);
                last = group.stream().into_iter();
                last.next().expect("TokenTree Group appears to be empty")
            }
            Some(token) => token,
        };
        Some((last, ret))
    }
}

impl From<TokenStream> for TokenStreamFlatten {
    fn from(stream: TokenStream) -> TokenStreamFlatten {
        TokenStreamFlatten {
            data: vec![stream.into_iter()],
        }
    }
}
