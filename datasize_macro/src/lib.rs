use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DataSize)]
pub fn derive_data_size(item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as DeriveInput);
	let ident = input.ident;
	let output = quote! {
		impl cornflakes::datasize::DataSize for #ident {
			fn data_size(&self) -> usize {
				usize::default()
			}
		}
	};
	TokenStream::from(output)
}

#[proc_macro_derive(StaticDataSize)]
pub fn derive_static_data_size(item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as DeriveInput);
	let ident = input.ident;
	let output = quote! {
		impl cornflakes::datasize::StaticDataSize for #ident {
			fn static_data_size() -> usize {
				usize::default()
			}
		}
		impl cornflakes::datasize::DataSize for #ident {
		    fn data_size(&self) -> usize {
		        Self::static_data_size()
		    }
		}
	};
	TokenStream::from(output)
}
