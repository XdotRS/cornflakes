// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use impl_data_sizes::{impl_datasize, impl_static_data_size};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
	parse_macro_input, parse_quote, DeriveInput,
};

mod impl_data_sizes;

#[proc_macro_derive(DataSize)]
pub fn derive_data_size(item: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(item as DeriveInput);
	let ident = &input.ident;
	let inner = impl_datasize(&input);

	input.generics.type_params_mut().for_each(|param| {
		param.bounds.push(parse_quote!(cornflakes::DataSize));
	});
	let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

	let output = quote! {
		impl #impl_generics cornflakes::DataSize for #ident #type_generics #where_clause {
			default fn data_size(&self) -> usize {
				#inner
			}
		}
	};
	output.into()
}

#[proc_macro_derive(StaticDataSize)]
pub fn derive_static_data_size(item: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(item as DeriveInput);
	let ident = &input.ident;
	let inner = impl_static_data_size(&input);

	input.generics.type_params_mut().for_each(|param| {
		param.bounds.push(parse_quote!(cornflakes::StaticDataSize));
	});
	let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

	let output = quote! {
		impl #impl_generics cornflakes::StaticDataSize for #ident #type_generics #where_clause {
			fn static_data_size() -> usize {
				#inner
			}
		}
	};

	// For specialization to work, we need an additionnal trait bound for Datasize
	// that we couldn't add before
	input.generics.type_params_mut().for_each(|param| {
		param.bounds.push(parse_quote!(cornflakes::DataSize));
	});
	let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

	let output = quote! {
		#output
		impl #impl_generics cornflakes::DataSize for #ident #type_generics #where_clause {
			fn data_size(&self) -> usize {
				<Self as cornflakes::StaticDataSize>::static_data_size()
			}
		}
	};
	output.into()
}
