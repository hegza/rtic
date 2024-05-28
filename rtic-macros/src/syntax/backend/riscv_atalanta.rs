use syn::{
    parse::{Parse, ParseStream},
    Error, Result,
};

#[derive(Debug)]
pub struct BackendArgs();

impl Parse for BackendArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Err(Error::new(
            input.span(),
            "riscv-atalanta backend does not accept any arguments",
        ))
    }
}
