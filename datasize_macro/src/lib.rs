use impl_data_sizes::{impl_datasize, impl_static_data_size};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod impl_data_sizes;

#[proc_macro_derive(DataSize)]
pub fn derive_data_size(item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as DeriveInput);
	let ident = &input.ident;
	let inner = impl_datasize(&input);

	let output = quote! {
		impl cornflakes::datasize::DataSize for #ident {
			fn data_size(&self) -> usize {
				#inner
			}
		}
	};
	output.into()
}

#[proc_macro_derive(StaticDataSize)]
pub fn derive_static_data_size(item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as DeriveInput);
	let ident = &input.ident;
	let inner = impl_static_data_size(&input);

	// TODO: Move the implementation for `DataSize` to the appropriate derive macro
	// TODO: Implement Generics support
	let output = quote! {
		impl cornflakes::datasize::StaticDataSize for #ident {
			fn static_data_size() -> usize {
				#inner
			}
		}
		impl cornflakes::datasize::DataSize for #ident {
			fn data_size(&self) -> usize {
				Self::static_data_size()
			}
		}
	};
	output.into()
}
