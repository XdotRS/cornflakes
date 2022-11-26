use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Ident, Index, PathArguments, Type};

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
		syn::Data::Union(_) => {
			panic!("Unions are used for C bindings, you probably don't need this trait for it");
		}
	}
}

fn impl_datasize_enum(data_enum: &DataEnum) -> TokenStream2 {
	let branches = data_enum
		.variants
		.iter()
		.map(|v| {
			let ident = &v.ident;
			match &v.fields {
				Fields::Named(f) => {
					// Get a list of all named fields of a named variant
					let names: Vec<Ident> =
						f.named.iter().filter_map(|f| f.ident.to_owned()).collect();

					quote! {
						Self::#ident {#(#names),*} => { usize::default() #( + #names.data_size())*},
					}
				}
				Fields::Unnamed(f) => {
					// Set a name for all fields of an unnamed variant
					let names: Vec<Ident> = f
						.unnamed
						.iter()
						.enumerate()
						.map(|(i, _)| format_ident!("f{}", i))
						.collect();

					quote! {
						Self::#ident (#(#names),*) => { usize::default() #( + #names.data_size())*},
					}
				}
				// If the variant is a unit variant, then the size is 0
				Fields::Unit => quote! {
					Self::#ident => 0,
				},
			}
		})
		.fold(quote!(), |t, b| quote! (#t #b));

	quote! {
		match &self {
			#branches
		}
	}
}

fn impl_datasize_struct(data_struct: &DataStruct) -> TokenStream2 {
	match &data_struct.fields {
		Fields::Named(f) => {
			// Get a list of all named fields of a named variant
			let names: Vec<Ident> = f.named.iter().filter_map(|f| f.ident.to_owned()).collect();

			quote! {
				usize::default() #(+ self.#names.data_size())*
			}
		}
		Fields::Unnamed(f) => {
			// Set a name for all fields of an unnamed variant
			let names: Vec<Index> = f
				.unnamed
				.iter()
				.enumerate()
				.map(|(i, _)| Index::from(i))
				.collect();

			quote! {
				usize::default() #(+ self.#names.data_size())*
			}
		}
		// If the variant is a unit variant, then the size is 0
		Fields::Unit => quote!(usize::default()),
	}
}

fn impl_static_datasize_enum(data_enum: &DataEnum) -> TokenStream2 {
	data_enum
		.variants
		.iter()
		.map(|v| {
			// Retrieve types for all fields
			let types: Vec<Type> = v.fields.iter().map(|f| f.ty.to_owned()).collect();
			quote! {
				usize::default() #( + #types::static_data_size())*
			}
		})
		// Use the maximum size among all variants
		.fold(
			quote!(usize::default()),
			|t, b| quote!(std::cmp::max(#t, #b)),
		)
}

fn impl_static_datasize_struct(data_struct: &DataStruct) -> TokenStream2 {
	// Retrieve types of all fields
	let types: Vec<_> = data_struct
		.fields
		.iter()
		.map(|f| f.ty.to_owned())
		.filter_map(|t| match t {
			Type::Path(p) => Some(p),
			_ => None,
		})
		// Replacing every paths segments angle brackets argument
		// Basically tansforming every Type<T> into Type::<T>
		.map(|p| {
			let idents: Vec<_> = p
				.path
				.segments
				.iter()
				.map(|s| {
					let ident = &s.ident;
					// Only retain angle btracketed arguments
					let PathArguments::AngleBracketed(arg) = &s.arguments else {
								return quote!(#ident);
							};
					quote!(#ident::#arg)
				})
				.collect();
			quote!(#(#idents)*)
		})
		.collect();

	// We call `static_data_size()` on each of the names
	quote! ( usize::default() #(+ #types::static_data_size())*)
}

pub fn retrieve_generics(input: &DeriveInput) -> Vec<Ident> {
	input
		.generics
		.type_params()
		.map(|t| t.ident.to_owned())
		.collect()
}
