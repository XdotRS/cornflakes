use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Index, PathArguments, Type};

#[proc_macro_derive(DataSize)]
pub fn derive_data_size(item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as DeriveInput);
	let ident = input.ident;

	let inner = match input.data {
		Data::Enum(e) => {
			let branches = e
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
		Data::Struct(s) => match &s.fields {
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
		},
		Data::Union(_) => {
			panic!("Unions are used for C bindings, you probably don't need this trait for it");
		}
	};

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
	let ident = input.ident;

	let inner = match input.data {
		Data::Enum(e) => {
			e.variants
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
		Data::Struct(s) => {
			// Retrieve types of all fields
			let types: Vec<_> = s
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
		syn::Data::Union(_) => {
			panic!("Unions are used for C bindings, you probably don't need this trait for it");
		}
	};

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
