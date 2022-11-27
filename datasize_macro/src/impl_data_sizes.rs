// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
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
					let names: Vec<Ident> = field
						.named
						.iter()
						.filter_map(|f| f.ident.to_owned())
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
			let names: Vec<Ident> = field
				.named
				.iter()
				.filter_map(|f| f.ident.to_owned())
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
			let types: Vec<Type> = variant.fields.iter().map(|f| f.ty.to_owned()).collect();
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
	let types: Vec<TokenStream2> = data_struct
		.fields
		.iter()
		.map(|field| field.ty.to_owned())
		.map(replace_type_syntax)
		.collect();

	// We call `static_data_size()` on each of the names
	quote! ( 0usize #(+ <#types>::static_data_size())*)
}

/// This replaces all types to a syntax on which we can call functions
fn replace_type_syntax(r#type: Type) -> TokenStream2 {
	match r#type {
		Type::Tuple(tuple) => {
			let types: Vec<TokenStream2> = tuple
				.elems
				.iter()
				.map(|r#type| replace_type_syntax(r#type.to_owned()))
				.collect();
			quote!((#(#types),*))
		}
		Type::Ptr(ptr) => {
			let ty = replace_type_syntax(*ptr.elem);
			quote!(#ty)
		}
		Type::Array(array) => {
			let ty = replace_type_syntax(*array.elem);
			quote!([#ty])
		}
		Type::Slice(slice) => {
			let ty = replace_type_syntax(*slice.elem);
			quote!([#ty])
		}
		Type::Reference(reference) => {
			let ty = replace_type_syntax(*reference.elem);
			quote!(#ty)
		}
		Type::Infer(i) => i.to_token_stream(),
		Type::Path(p) => p.to_token_stream(),
		_ => panic!("This type is not supported yet"),
	}
}

pub fn retrieve_generics(input: &DeriveInput) -> Vec<Ident> {
	input
		.generics
		.type_params()
		.map(|r#type| r#type.ident.to_owned())
		.collect()
}

pub fn retrieve_lifetimes(input: &DeriveInput) -> Vec<TokenStream2> {
	input.generics.lifetimes().map(|_| quote!('_)).collect()
}
