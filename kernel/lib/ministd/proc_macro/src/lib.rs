#![no_std]

use proc_macro::*;

use proc_macro2::TokenStream as TokenStream2;
use quote::{TokenStreamExt, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::token::Pound;
use syn::{
    parse_macro_input, parse_quote, parse_quote_spanned, Attribute, Error, Expr, ExprLit, ExprPath, FnArg, ItemFn, ItemStruct, Lit, Path, ReturnType, Type, Visibility
};
use syn::{PatType, TypeReference, TypePath, Signature, GenericArgument};



const EXPECTED_FN_ARGS: ([&'static str; 2], [&'static str; 2]) = (["ministd", "HeapRef"], ["ministd", "Allocator"]);


/// Defines the entry point of the kernel
/// - the function must never return (returns `!`) and must not take any parameters
#[proc_macro_attribute]
pub fn entry(_: TokenStream, input: TokenStream) -> TokenStream {

    let mut f = parse_macro_input!(input as ItemFn);
    
    if let Some(_) = f.sig.abi {
        panic!("Entry function must have no ABI modifier");
    }


    if let Some(_) = f.sig.asyncness {
        panic!("Entry function cannot be async");
    }


    if let Some(_) = f.sig.constness {
        panic!("Entry function cannot be const");
    }

    if !f.sig.generics.params.is_empty() {
        panic!("Entry function cannot have any generic parameters");
    }

    if !f.sig.inputs.is_empty() {
        panic!("Entry function cannot take any arguments");
    }


    match &f.sig.output {
        ReturnType::Type(_, ty) => {
            match ty.as_ref() {
                Type::Never(_) => {
                    //  OK
                },
                _ => {
                    panic!("iniEntry function must never return (add `-> !`)");
                }
            }
        },
        _ => {
            panic!("Entry function must never return (add `-> !`)");
        }
    }

    f.sig.abi = Some(syn::Abi {
        extern_token: Default::default(),
        name: Some(syn::LitStr::new("C", proc_macro2::Span::call_site())),
    });

    f.attrs.push(parse_quote!(#[unsafe(no_mangle)]));
    f.attrs.push(parse_quote!(#[unsafe(export_name = "_start")]));

    let func: TokenStream = quote!(#f).into();

    func

}


#[proc_macro_attribute]
pub fn oom(_: TokenStream, input: TokenStream) -> TokenStream {


    let mut f = parse_macro_input!(input as ItemFn);

    let sig = &f.sig;
    check_signature(sig);

    let first = sig.inputs.get(0).expect("failed to get first argument");

    //  check first argument
    if let Err(o) = check_arg(first, true, &EXPECTED_FN_ARGS.0) {
        if let Some(s) = o {
            panic!("{s}");
        } else {
            panic!("first argument must be of type &mut ministd::HeapRef");
        }
    }

    let second = sig.inputs.get(1).expect("failed to get first argument");
    //  check second argument
    if let Err(o) = check_arg(second, false, &EXPECTED_FN_ARGS.1) {
        if let Some(s) = o {
            panic!("{s}");
        } else {
            panic!("second argument must be of type &ministd::Allocator");
        }
    }

    check_return_type(&sig.output);

    f.sig.abi = Some(syn::Abi {
        extern_token: Default::default(),
        name: Some(syn::LitStr::new("Rust", proc_macro2::Span::call_site())),
    });

    f.attrs.push(parse_quote!(#[unsafe(no_mangle)]));
    f.attrs.push(parse_quote!(#[unsafe(export_name = "__oom_handler")]));
    

    let func: TokenStream = quote!(#f).into();

    func




}

fn check_return_type(output: &ReturnType) {
    let path = match output {
        ReturnType::Type(_, ty) => {
            if let Type::Path(TypePath { path, .. }) = &**ty {
                path
            } else {
                panic!("OMM handler must never return (add -> !)")
            }
        },
        _ => panic!("OMM handler must never return (add -> !)"),
    };

    if path.segments.len() != 1 {
        panic!("OMM handler is required to return `Result<(), ()>`");
    }


    let result_segment = path.segments.last().expect("cannot get last generci argument of the returntype");
    // Check for generic arguments: <(), ()>
    if let syn::PathArguments::AngleBracketed(args) = &result_segment.arguments {
        let mut args = args.args.iter();

        let Some(GenericArgument::Type(Type::Tuple(ok))) = args.next() else {
            panic!("OMM handler is required to return `Result<(), ()>`");
        };
        if !ok.elems.is_empty() {
            panic!("OMM handler is required to return `Result<(), ()>`");
        }

        let Some(GenericArgument::Type(Type::Tuple(err))) = args.next() else { 
            panic!("OMM handler is required to return `Result<(), ()>`");
        };
        if !err.elems.is_empty() {
            panic!("OMM handler is required to return `Result<(), ()>`");
        }
    }
}


fn check_signature(sig: &Signature) {
    if let Some(_) = sig.abi {
        panic!("OOM handler cannot have any ABI set");
    }

    if let Some(_) = sig.asyncness {
        panic!("OOM handler cannot be async");
    }

    if let Some(_) = sig.constness {
        panic!("OOM handler cannot be constant");
    }

    if let Some(_) = sig.unsafety {
        panic!("OOM handler cannot be unsafe");
    }

    if sig.inputs.len() != 2 {
        panic!("OOM handler have to have exactly 2 arguments of type `&mut ministd::HeapRef` and `&ministd::AllocatorRef`");
    }

    if let Some(_) = sig.generics.gt_token {
        panic!("OOM handler cannot have any generic arguments")
    }

}
    
fn check_arg(arg: &FnArg, mutable: bool, expected: &[&'static str]) -> Result<(), Option<&'static str>> {
    let FnArg::Typed(PatType { ty , ..}) = arg else {
        return Err(None)
    };


    let path = if let Type::Reference(TypeReference {
        mutability, elem, ..}) = &**ty {
        
        if mutable != mutability.is_some() {
            if mutable {
                panic!("make the argument a mutable reference")
            } else {
                panic!("the argument cannot be mutable reference")
            }
        }

        let Type::Path(TypePath { path: type_path, .. }) = &**elem else {
            return Err(None)
        };
        
        type_path
    } else {
        panic!("the argument must be a mutable reference")
    };

    match path.segments.len() {
        1 => {
            let seg = path.segments.get(0).expect("unexpected internal error 1")
                .ident.span().source_text().expect("unexpected internal error 2");
            if seg != expected[1] {
                return Err(None)
            }
        },
        2 => {
            for (i, seg) in path.segments.iter().enumerate() {
                let s = if let Some(s) = seg.ident.span().source_text() {
                    s
                } else {
                    return Err(None)
                };
                //let s = seg.ident.span().source_text().expect("unexpected internal error 3");
                if s != expected[i] {
                    return Err(None)
                }

            }
        }
        _ => return Err(None)
    }

    Ok(())
}