use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
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
	let types: Vec<TokenStream2> = data_struct
		.fields
		.iter()
		.map(|f| f.ty.to_owned())
		.map(replace_type_syntax)
		.collect();

	// We call `static_data_size()` on each of the names
	quote! ( usize::default() #(+ <#types>::static_data_size())*)
}

/// This replaces all types to a syntax on which we can call functions
fn replace_type_syntax(t: Type) -> TokenStream2 {
	match t {
		// If it is a path type, we need to replace its arguments is there is some
		// Basically tansforming every Type<T> into Type::<T>
		Type::Path(p) => {
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
					// TODO: Support recursive arguments (Things like Option<Thing<T>>)
					quote!(#ident::#arg)
				})
				.collect();
			quote!(#(#idents)*)
		}
		Type::Tuple(t) => {
			let types: Vec<TokenStream2> = t
				.elems
				.iter()
				.map(|t| replace_type_syntax(t.to_owned()))
				.collect();
			quote!((#(#types),*))
		}
		Type::Ptr(p) => {
			let ty = replace_type_syntax(*p.elem);
			quote!(#ty)
		}
		Type::Array(a) => {
			let ty = replace_type_syntax(*a.elem);
			quote!([#ty])
		}
		Type::Slice(s) => {
			let ty = replace_type_syntax(*s.elem);
			quote!([#ty])
		}
		Type::Reference(r) => {
			let ty = replace_type_syntax(*r.elem);
			quote!(#ty)
		}
		Type::Infer(i) => i.to_token_stream(),
		_ => panic!("This type is not supported yet"),
	}
}

pub fn retrieve_generics(input: &DeriveInput) -> Vec<Ident> {
	input
		.generics
		.type_params()
		.map(|t| t.ident.to_owned())
		.collect()
}

pub fn retrieve_lifetimes(input: &DeriveInput) -> Vec<TokenStream2> {
	input.generics.lifetimes().map(|_| quote!('_)).collect()
}
