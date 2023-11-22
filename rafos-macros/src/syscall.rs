use bae::FromAttributes;
use syn;

#[derive(Default, FromAttributes)]
pub struct Arguments {
    pub args: Option<syn::LitStr>,
}
