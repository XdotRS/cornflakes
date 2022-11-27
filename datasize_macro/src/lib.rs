// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use impl_data_sizes::{
	impl_datasize, impl_static_data_size, retrieve_generics_with_anonymous_lifetimes, retrieve_type_generics,
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

	let generics = retrieve_type_generics(&input);
	let generics_and_anonymous_lifetimes = retrieve_generics_with_anonymous_lifetimes(&input);

	let output = quote! {
		impl<#(#generics: cornflakes::DataSize),*> cornflakes::DataSize for #ident <#(#generics_and_anonymous_lifetimes),*> {
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

	let generics = retrieve_type_generics(&input);
	let generics_and_anonymous_lifetimes = retrieve_generics_with_anonymous_lifetimes(&input);

	// TODO: Move the implementation for `DataSize` to the appropriate derive macro
	let output = quote! {
		impl<#(#generics: cornflakes::StaticDataSize),*> cornflakes::StaticDataSize for #ident <#(#generics_and_anonymous_lifetimes),*> {
			fn static_data_size() -> usize {
				#inner
			}
		}
		impl<#(#generics: cornflakes::StaticDataSize + cornflakes::DataSize),*> cornflakes::DataSize for #ident <#(#generics_and_anonymous_lifetimes),*> {
			fn data_size(&self) -> usize {
				Self::static_data_size()
			}
		}
	};
	output.into()
}
