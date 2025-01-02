use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, ItemTrait};

#[proc_macro_attribute]
pub fn generate_static(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;

    let expanded = quote! {
        #input
        
        #[allow(non_upper_case_globals)]
        #[linkme::distributed_slice]
        pub static #trait_name: [usize] = [..];
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn use_static(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let trait_name = if let Some((_, path, _)) = &input.trait_ {
        path.segments.last().unwrap().ident.clone()
    } else {
        panic!("use_static can only be applied to trait implementations");
    };

    let expanded = quote! {
        #input
        
        #[linkme::distributed_slice(#trait_name)]
        static REGISTER: usize = 0;
    };

    TokenStream::from(expanded)
}
