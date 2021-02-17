use syn::{Result, parse::Parse, parse::ParseStream, Ident};

pub struct AttributeArgs {
    pub type_struct: Ident,
    pub fields_defined: Option<Vec<(Ident, Ident)>>
}

impl Parse for AttributeArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let type_struct: syn::Ident = input.parse::<syn::Ident>()?;
        let content;
        if input.is_empty() {
            Ok( 
                AttributeArgs {
                    type_struct: type_struct,
                    fields_defined: None
                }
            )
        } else {
            input.parse::<syn::Ident>()?;
            syn::parenthesized!(content in input);
            Ok( 
                AttributeArgs {
                    type_struct: type_struct,
                    fields_defined: Some({
                        let mut fields_defined = Vec::new();
                        while !content.is_empty() {
                            let test_fields: (Ident, syn::token::Eq, Ident) = (content.parse()?, content.parse()?, content.parse()?);
                            fields_defined.push((test_fields.0, test_fields.2));
                        }
                        fields_defined
                    })
                }
            )
        }
    }
}