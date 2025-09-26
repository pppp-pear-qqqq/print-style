use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(AnsiStylize)]
pub fn derive(input: TokenStream) -> TokenStream {
	let input = &parse_macro_input!(input as DeriveInput);

	match generate(input) {
		Ok(generated) => generated,
		Err(err) => err.to_compile_error().into(),
	}
}

fn generate(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
	// enum情報を取得
	let data = match &input.data {
		syn::Data::Enum(v) => v,
		_ => {
			return Err(syn::Error::new_spanned(&input.ident, "enum型が必要です"));
		}
	};
	// 各値を取得してTokenStreamに変換
	let mut variants = Vec::new();
	for v in &data.variants {
		if !v.fields.is_empty() {
			return Err(syn::Error::new_spanned(&v.fields, "フィールドは存在できません"));
		}
		let ident = &v.ident;
		variants.push(quote! {
			fn #ident(&self) -> Styled<'_, Self> {
				self.with(vec![Effect::Style(Ansi::#ident)], true)
			}
		});
	}
	let expanded = quote! {
		pub trait AnsiStylize: Stylize {
			#(#variants)*
		}
		impl<T: Stylize> AnsiStylize for T {}
	};
	Ok(expanded.into())
}
