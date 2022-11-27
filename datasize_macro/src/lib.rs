// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use impl_data_sizes::{
	impl_datasize, impl_static_data_size, retrieve_generics, retrieve_lifetimes,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod impl_data_sizes;

#[proc_macro_derive(DataSize)]
pub fn derive_data_size(item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as DeriveInput);
	let ident = &input.ident;
	let inner = impl_datasize(&input);

	// Parsing generics and lifetimes
	let generics = retrieve_generics(&input);
	let lifetimes = retrieve_lifetimes(&input);
	// Here we check if there is any lifetime AND generics
	// if there is both, we need a comma to separate them
	let generics_and_lifetimes = if generics.len() > 0 && lifetimes.len() > 0 {
		quote!(<#(#lifetimes),*, #(#generics)*>)
	} else {
		quote!(<#(#lifetimes),* #(#generics)*>)
	};

	let output = quote! {
		impl<#(#generics: DataSize),*> cornflakes::datasize::DataSize for #ident #generics_and_lifetimes {
			default fn data_size(&self) -> usize {
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

	// Parsing generics and lifetimes
	let generics = retrieve_generics(&input);
	let lifetimes = retrieve_lifetimes(&input);
	// Here we check if there is any lifetime AND generics
	// if there is both, we need a comma to separate them
	let generics_and_lifetimes = if generics.len() > 0 && lifetimes.len() > 0 {
		quote!(<#(#lifetimes),*, #(#generics)*>)
	} else {
		quote!(<#(#lifetimes),* #(#generics)*>)
	};

	// TODO: Move the implementation for `DataSize` to the appropriate derive macro
	let output = quote! {
		impl<#(#generics: StaticDataSize),*> cornflakes::datasize::StaticDataSize for #ident #generics_and_lifetimes {
			fn static_data_size() -> usize {
				#inner
			}
		}
		impl<#(#generics: StaticDataSize + DataSize),*> cornflakes::datasize::DataSize for #ident #generics_and_lifetimes {
			fn data_size(&self) -> usize {
				Self::static_data_size()
			}
		}
	};
	output.into()
}
