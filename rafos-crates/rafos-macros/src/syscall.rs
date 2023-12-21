use bae::FromAttributes;

#[derive(FromAttributes)]
pub struct Arguments {
    pub a0: Option<syn::FnArg>,
    pub a1: Option<syn::FnArg>,
    pub a2: Option<syn::FnArg>,
    pub a3: Option<syn::FnArg>,
    pub a4: Option<syn::FnArg>,
    pub a5: Option<syn::FnArg>,
    pub a6: Option<syn::FnArg>,
}
