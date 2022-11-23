use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Ident, PathArguments, Type};

// mod definitions;

#[proc_macro_derive(DataSize)]
pub fn derive_data_size(item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as DeriveInput);
	let ident = input.ident;

	let inner = match input.data {
		syn::Data::Enum(e) => {
			let branches = e
				.variants
				.iter()
				.map(|v| {
					let ident = &v.ident;
					let fields_named = v.fields.iter().all(|v| v.ident.is_some());
					if fields_named {
						// Get a list of all named fields of the variant
						let names: Vec<Ident> = v
							.fields
							.iter()
							.filter_map(|f| {
								if let Some(name) = &f.ident {
									Some(format_ident!("{}", name.to_string()))
								} else {
									None
								}
							})
							.collect();

						quote! {
							Self::#ident {#(#names),*} => { usize::default() #( + #names.data_size())*},
						}
					} else if v.fields.len() > 0 {
						// Set a name for all fields of a variant
						let names: Vec<Ident> = v
							.fields
							.iter()
							.enumerate()
							.map(|(i, _)| format_ident!("f{}", i))
							.collect();

						quote! {
							Self::#ident (#(#names),*) => { usize::default() #( + #names.data_size())*},
						}
					} else {
						// If the variant is a unit variant, then the size is 0
						quote! {
							Self::#ident => 0,
						}
					}
				})
				.fold(quote!(), |t, b| quote! (#t #b));

			quote! {
				match &self {
					#branches
				}
			}
		}
		syn::Data::Struct(s) => {
			// Retreive the names of all the fields
			let names: Vec<Ident> = s
				.fields
				.iter()
				.enumerate()
				.map(|(i, f)| {
					if let Some(ident) = &f.ident {
						format_ident!("{}", ident.to_string())
					} else {
						format_ident!("{}", i)
					}
				})
				.collect();

			// We call `data_size()` on each of the names
			quote! { usize::default() #(+ self.#names.data_size())*}
		}
		syn::Data::Union(_) => {
			// TODO: find you what to do here
			quote! {0}
		}
	};

	let output = quote! {
		impl cornflakes::datasize::DataSize for #ident {
			fn data_size(&self) -> usize {
				#inner
			}
		}
	};
	TokenStream::from(output)
}

#[proc_macro_derive(StaticDataSize)]
pub fn derive_static_data_size(item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as DeriveInput);
	let ident = input.ident;

	let inner = match input.data {
		syn::Data::Enum(e) => {
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
					|t, b| quote! (std::cmp::max(#t, #b)),
				)
		}
		syn::Data::Struct(s) => {
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
							let PathArguments::AngleBracketed(arg) = &s.arguments else {
								return quote!(format_ident!("{}", s.ident));
							};
							let arg = arg.to_token_stream();
							quote!(format_ident!("{}", s.ident)::#arg)
						})
						.collect();
					quote!(#(::#idents)*)
				})
				.collect();

			// We call `static_data_size()` on each of the names
			quote! ( usize::default() #(+ #types::static_data_size())*)
		}
		syn::Data::Union(_) => {
			// TODO: find you what to do here
			quote! {0}
		}
	};

	// TODO: Move the implementation for `DataSize` to the appropriate derive macro
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
	TokenStream::from(output)
}
