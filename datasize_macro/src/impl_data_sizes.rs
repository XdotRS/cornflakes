// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Ident, Index, Type};

pub fn impl_datasize(input: &DeriveInput) -> TokenStream2 {
	match &input.data {
		Data::Enum(e) => impl_datasize_enum(e),
		Data::Struct(s) => impl_datasize_struct(s),
		Data::Union(_) => {
			panic!("Unions are used for C bindings, you probably don't need this trait for it");
		}
	}
}

pub fn impl_static_data_size(input: &DeriveInput) -> TokenStream2 {
	match &input.data {
		Data::Enum(e) => impl_static_datasize_enum(e),
		Data::Struct(s) => impl_static_datasize_struct(s),
		Data::Union(_) => {
			panic!("Unions are used for C bindings, you probably don't need this trait for it");
		}
	}
}

fn impl_datasize_enum(data_enum: &DataEnum) -> TokenStream2 {
	let branches = data_enum
		.variants
		.iter()
		.map(|variant| {
			let ident = &variant.ident;
			match &variant.fields {
				Fields::Named(field) => {
					// Get a list of all named fields of a named variant
					let names: Vec<&Ident> = field
						.named
						.iter()
						.filter_map(|f| f.ident.as_ref())
						.collect();

					quote! {
						Self::#ident {#(#names),*} => { 0usize #( + #names.data_size())*},
					}
				}
				Fields::Unnamed(field) => {
					// Set a name for all fields of an unnamed variant
					let names: Vec<Ident> = (0..(field.unnamed.len()))
						.map(|index| format_ident!("field{}", index))
						.collect();

					quote! {
						Self::#ident (#(#names),*) => { 0usize #( + #names.data_size())*},
					}
				}
				// If the variant is a unit variant, then the size is 1 (size of it's discriminant)
				Fields::Unit => quote! {
					Self::#ident => 1,
				},
			}
		})
		.fold(
			TokenStream2::new(),
			|tokens, branch| quote! (#tokens #branch),
		);

	quote! {
		match &self {
			#branches
		}
	}
}

fn impl_datasize_struct(data_struct: &DataStruct) -> TokenStream2 {
	match &data_struct.fields {
		Fields::Named(field) => {
			// Get a list of all named fields of a named variant
			let names: Vec<&Ident> = field
				.named
				.iter()
				.filter_map(|f| f.ident.as_ref())
				.collect();

			quote! {
				0usize #(+ self.#names.data_size())*
			}
		}
		Fields::Unnamed(field) => {
			// Set a name for all fields of an unnamed variant
			let names: Vec<Index> = (0..(field.unnamed.len()))
				.map(|index| Index::from(index))
				.collect();

			quote! {
				0usize #(+ self.#names.data_size())*
			}
		}
		// If the variant is a unit variant, then the size is 0
		Fields::Unit => quote!(0usize),
	}
}

fn impl_static_datasize_enum(data_enum: &DataEnum) -> TokenStream2 {
	data_enum
		.variants
		.iter()
		.map(|variant| {
			// Retrieve types for all fields
			let types: Vec<&Type> = variant.fields.iter().map(|f| &f.ty).collect();
			quote! {
				0usize #( + #types::static_data_size())*
			}
		})
		// Use the maximum size among all variants
		.fold(
			quote!(0usize),
			|tokens, branch| quote!(std::cmp::max(#tokens, #branch)),
		)
}

fn impl_static_datasize_struct(data_struct: &DataStruct) -> TokenStream2 {
	// Retrieve types of all fields
	let types: Vec<&Type> = data_struct.fields.iter().map(|field| &field.ty).collect();

	// We call `static_data_size()` on each of the names
	quote! ( 0usize #(+ <#types>::static_data_size())*)
}
