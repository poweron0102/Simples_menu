extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(BoundingRect)]
pub fn bounding_rect(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let ast = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct.
    let name = &ast.ident;

    // Get the fields of the struct.
    let fields = match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    // Check that the struct has the required fields.
    let has_visible = fields.iter().any(|field| field.ident.as_ref().unwrap() == "visible");
    let has_position = fields.iter().any(|field| field.ident.as_ref().unwrap() == "position");
    let has_size = fields.iter().any(|field| field.ident.as_ref().unwrap() == "size");
    if !(has_visible && has_position && has_size) {
        panic!("Struct {} does not have the required fields", name);
    }

    // Generate the code for the bounding_rect method.
    let gen = quote! {
        impl #name {
            fn bounding_rect(&self) -> Option<Rect> {
                if self.visible {
                    Some(Rect::new(self.position.x, self.position.y, self.size.x, self.size.y))
                } else {
                    None
                }
            }
        }
    };

    // Return the generated code as a TokenStream.
    gen.into()
}
