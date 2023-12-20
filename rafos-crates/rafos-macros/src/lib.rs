#![allow(unused_assignments)]

mod syscall;

extern crate alloc;
extern crate proc_macro;
use alloc::vec::Vec;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{parse_macro_input, DeriveInput, Ident, ItemFn};
use syscall::Arguments;

#[proc_macro_attribute]
pub fn entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let blocks = input_fn.block.stmts;
    let mut statements = Vec::new();
    for stmt in blocks {
        statements.push(stmt.clone().to_token_stream());
    }
    // println!("{:?}", args_value[0]);
    let mut derive_fn = TokenStream::default();
    derive_fn = quote!(
        #[macro_use]
        extern crate console;
        extern crate alloc;
        use alloc::boxed::Box;
        extern crate syscall;
        use syscall::*;

        #[no_mangle]
        #[link_section = ".text.entry"]
        pub fn _start() -> ! {
            console::init(option_env!("LOG"));
            init_heap();
            sys_exit(main() as usize);
            panic!()
        }

        fn main() -> isize {
            #(#statements)*
        }

        #[no_mangle]
        pub extern "C" fn put_str(ptr: *const u8, len: usize) {
            sys_write(1, ptr as _, len);
        }

        pub fn getchar() -> u8 {
            let mut c = [0u8; 1];
            let mut res = -1;
            while res < 0 {
                res = sys_read(0, c.as_ptr() as usize, c.len());
            }
            c[0]
        }


        use buddy_system_allocator::LockedHeap;
        use core::{
            alloc::Layout,
            ptr::NonNull,
        };

        #[no_mangle]
        #[link_section = ".data.heap"]
        #[global_allocator]
        pub static mut HEAP: LockedHeap<32> = LockedHeap::new();

        pub const USER_HEAP_SIZE: usize = 0x40000;

        #[no_mangle]
        #[link_section = ".bss.memory"]
        static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0u8; USER_HEAP_SIZE];

        /// 
        fn init_heap() {
            unsafe {
                HEAP.lock().init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
            }
        }
        ///
        #[lang = "eh_personality"]
        #[no_mangle]
        pub fn rust_eh_personality() {}

        #[no_mangle]
        pub fn _Unwind_Resume() {}



        #[panic_handler]
        fn panic(info: &core::panic::PanicInfo) -> ! {
            println!("{}", info);
            sys_exit(usize::MAX);
            panic!("")
        }
    ).into();
    // println!("{}", derive_fn.to_string());
    derive_fn
}

/// function instrumentation
#[proc_macro]
pub fn get_libfn(item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    // let attrs: Vec<_> = input_fn.attrs.into_iter().map(|attr| attr.parse_meta().ok()).collect();
    let ident = input_fn.sig.ident;
    let args = input_fn.sig.inputs.to_token_stream();
    let mut args_str = args.to_string();
    args_str.push(',');
    let re = Regex::new(r"(?s):[^,]*,").unwrap();
    let mut args_type_str = re.replace_all(args_str.as_str(), r",").to_string();
    args_type_str.push(' ');
    let mut args_value: Vec<_> = args_type_str.split(" , ").collect();
    args_value.pop();
    let args_value: Vec<syn::Ident> = args_value.iter().map(|s| Ident::new(*s, Span::call_site())).collect();
    let output = input_fn.sig.output.to_token_stream();
    let mut derive_fn = TokenStream::default();
    let fn_stub = quote::format_ident!("{}_stub", ident);
    derive_fn = quote!(
        #[no_mangle]
        #[link_section = ".dependency"]
        pub static mut #fn_stub: usize = 0;
        #[inline(never)]
        pub fn #ident(#args) #output {
            unsafe {
                let func: fn(#args) #output = core::mem::transmute(#fn_stub);
                func(#(#args_value),*)
            }
        }
    ).into();
    // println!("{}", derive_fn.to_string());
    derive_fn
}



/// user syscall interface
#[proc_macro_derive(GenSysMacro, attributes(arguments))]
pub fn syscall_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;
    let mut comment_arms = Vec::new();
    if let syn::Data::Enum(syn::DataEnum { variants, .. }) = input.data {
        for variant in variants {
            let ident_item = &variant.ident;
            let re = Regex::new(r"(?P<up>[A-Z])").unwrap();
            let before = ident_item.to_string();
            let mut fn_name_str = re.replace_all(before.as_str(), r"_$up").to_lowercase();
            fn_name_str.insert_str(0, "sys");
            // println!("{}", fn_name_str);
            let ident_name = syn::Ident::new(fn_name_str.as_str(), Span::call_site());
            if let Ok(args) = Arguments::from_attributes(&variant.attrs) {
                let args_vec: Vec<syn::Ident> = args
                    .args
                    .unwrap()
                    .value()
                    .split(", ")
                    .map(|s| syn::Ident::new(s, Span::call_site()))
                    .collect();
                let len = args_vec.len();
                let syscall_fn = quote::format_ident!("syscall{}", len);
                let mut doc = String::from("arguments are: ");
                for idx in 0..(len - 1) {
                    doc.push_str(&args_vec[idx].to_string().as_str());
                    doc.push_str(": usize, ");
                }
                doc.push_str(&args_vec[len - 1].to_string().as_str());
                doc.push_str(": usize");
                comment_arms.push(quote! (
                    #[doc = #doc]
                    #[inline]
                    pub fn #ident_name(#(#args_vec: usize), *) -> isize {
                        unsafe {
                            #syscall_fn(#ident::#ident_item as usize, #(#args_vec),*)
                        }
                    }
                ));
            } else {
                comment_arms.push(quote! (
                    #[inline]
                    pub fn #ident_name() -> isize {
                        unsafe {
                            syscall0(#ident::#ident_item as usize)
                        }
                    }
                ));
            }
        }
    }
    quote!(
        #(#comment_arms)*
        macro_rules! syscall {
            ($($name:ident($a:ident, $($b:ident, $($c:ident, $($d:ident, $($e:ident, $($f:ident, $($g:ident)?)?)?)?)?)?);)+) => {
                $(
                    #[inline]
                    pub unsafe fn $name($a: usize, $($b: usize, $($c: usize, $($d: usize, $($e: usize, $($f: usize, $($g: usize)?)?)?)?)?)?) -> isize {
                        let ret: isize;
                        core::arch::asm!(
                            "ecall",
                            in("a7") $a,
                            $(
                                in("a0") $b,
                                $(
                                    in("a1") $c,
                                    $(
                                        in("a2") $d,
                                        $(
                                            in("a3") $e,
                                            $(
                                                in("a4") $f,
                                                $(
                                                    in("a5") $g,
                                                )?
                                            )?
                                        )?
                                    )?
                                )?
                            )?
                            lateout("a0") ret,
                            options(nostack),
                        );
                        ret
                    }
                )+
            };
        }
        syscall! {
            syscall0(a,);
            syscall1(a, b,);
            syscall2(a, b, c,);
            syscall3(a, b, c, d,);
            syscall4(a, b, c, d, e,);
            syscall5(a, b, c, d, e, f,);
            syscall6(a, b, c, d, e, f, g);
        }
    ).into()
}

/// kernel syscall trait
#[proc_macro_derive(GenSysTrait, attributes(arguments))]
pub fn syscall_trait_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let mut trait_fns = Vec::new();
    if let syn::Data::Enum(syn::DataEnum { variants, .. }) = input.data {
        for variant in variants {
            let ident_item = &variant.ident;
            let re = Regex::new(r"(?P<up>[A-Z])").unwrap();
            let before = ident_item.to_string();
            let mut fn_name_str = re.replace_all(before.as_str(), r"_$up").to_lowercase();
            fn_name_str.insert_str(0, "sys");
            // println!("{}", fn_name_str);
            let ident_name = syn::Ident::new(fn_name_str.as_str(), Span::call_site());
            if let Ok(args) = Arguments::from_attributes(&variant.attrs) {
                let args_vec: Vec<syn::Ident> = args
                    .args
                    .unwrap()
                    .value()
                    .split(", ")
                    .map(|s| syn::Ident::new(s, Span::call_site()))
                    .collect();
                trait_fns.push(quote!(
                    #[inline]
                    fn #ident_name(#(#args_vec: usize),*) -> SyscallResult {
                        unimplemented!()
                    }
                ));
            } else {
                trait_fns.push(quote!(
                    #[inline]
                    fn #ident_name() -> SyscallResult {
                        unimplemented!()
                    }
                ));
            }
        }
    }
    quote!(
        pub trait SyscallTrait: Sync {
            #(#trait_fns)*
        }
    )
    .into()
}

/// sync & async fn
#[proc_macro_attribute]
pub fn async_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let name = input_fn.sig.ident;
    let mut sync_args = input_fn.sig.inputs.clone();
    sync_args.pop();
    sync_args.pop();
    let args = input_fn.sig.inputs.to_token_stream();
    let inner = input_fn.block;
    let mut args_str = args.to_string();
    args_str.push(',');
    let re = Regex::new(r"(?s):[^,]*,").unwrap();
    let mut args_type_str = re.replace_all(args_str.as_str(), r",").to_string();
    args_type_str.push(' ');
    let mut args_value: Vec<_> = args_type_str.split(" , ").collect();
    args_value.pop();
    args_value.pop();
    args_value.pop();
    // println!("{:?}", args_value);
    let args_value: Vec<syn::Ident> = args_value
        .iter()
        .map(|s| Ident::new(*s, Span::call_site()))
        .collect();
    // println!("{:?}", sync_args.to_token_stream().to_string());
    // println!("{:?}", args.to_string());

    let mut output = input_fn.sig.output.to_token_stream();
    if output.is_empty() {
        output.extend(quote!(-> ()));
    }
    let mut async_helper = proc_macro2::TokenStream::default();
    if let Ok(flag) = attr.to_string().parse::<bool>() {
        if flag {
            // println!("{:?}", flag);
            async_helper.extend(quote!(
                let async_call = $crate::AsyncCall::new();
                async_call.await;
            ));
        }
    }
    let derive_macro: TokenStream = quote!(
        #[inline(always)]
        pub fn #name(#args) #output #inner
        #[macro_export]
        macro_rules! #name {
            // sync
            (#($#args_value: expr),*) => {
                syscall::#name(#($#args_value),*, usize::MAX, usize::MAX)
            };
            // async
            (#($#args_value: expr),*, $key: expr, $cid: expr) => {
                syscall::#name(#($#args_value),*, $key, $cid);
                #async_helper
            }
        }
    )
    .into();
    derive_macro
}
