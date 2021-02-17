use syn::{Result, parse::Parse, parse::ParseStream, Ident};

pub struct AttributeArgs {
    pub type_struct: Ident,
    pub fields_defined: Option<Vec<(Ident, Ident)>>
}

impl Parse for AttributeArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let test: syn::Ident = input.parse::<syn::Ident>()?;
        //println!("test3 = {:#?}", test);
        Ok( 
            AttributeArgs {
                type_struct: test,
                fields_defined: None
            }
        )
    }
}