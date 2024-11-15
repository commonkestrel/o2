
use std::sync::Arc;

use async_std::fs::File;
use logos::Logos;

use crate::{diagnostic::Diagnostic, span::{Lookup, Spanned}, symbol_table::SymbolTable};

pub type TokenStream = Vec<Spanned<Token>>;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error = Diagnostic)]
#[logos(extras = SymbolTable)]
#[logos(skip r"[ \t\f\n\r]")]
#[logos(skip r"//[^!][^\n]*\n?")]
#[logos(skip r"/\*(?:[^*]|\*[^/])*\*/")]
pub enum Token {

}

pub struct LexResult {
    pub stream: TokenStream,
    pub source: Arc<String>,
    pub lookup: Arc<Lookup>,
    pub symbol_table: SymbolTable,
}

pub async fn lex(
    symbol_table: SymbolTable,
    source: String,
    content: File,
) -> Result<LexResult, Vec<Diagnostic>> {
    todo!()
}
