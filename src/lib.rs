#![recursion_limit="128"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use syn::{
    Type, Ident, Lit,
    parse::{Parse, ParseStream, Result}
};

mod parse;
use parse::*;

#[proc_macro]
pub fn make_optimizer(item: TokenStream) -> TokenStream {
    let Optimizer {struct_name, vars, evaluate} = parse_macro_input!(item as Optimizer);

    let names = vars.0.iter().map(|x| x.name.clone());
    let names2 = names.clone();
    let names3 = names.clone();
    let names4 = names.clone();
    let names5 = names.clone();

    let types = vars.0.iter().map(|x| x.ty.clone());
    let types2 = types.clone();
    let types3 = types.clone();

    let lows = vars.0.iter().map(|x| x.low.clone());
    let highs = vars.0.iter().map(|x| x.high.clone());

    let result = TokenStream::from(quote! {
        #[derive(Clone, Debug)]
        pub struct #struct_name {
            #( pub #names: #types ),*
        }
        impl #struct_name {
            pub fn evaluate(&self) -> f64 {
                let Self {#( #names4 ),*} = *self;
                #evaluate
            }

            pub fn random_search(max_iter: usize) -> #struct_name {
                use rand::distributions::{Uniform, Distribution};
                let mut rng = rand::thread_rng();
                let mut config = #struct_name {
                    #( #names2: #types2::default() ),*
                };

                let mut best_score = std::f64::NEG_INFINITY;
                let mut best_config = config.clone();
                let mut i = 0;
                loop {
                    // Sample random configuration
                    #(
                        config.#names5 = Uniform::new(#lows, #highs).sample(&mut rng);
                    )*
                    // Evaluate
                    let score = config.evaluate();
                    if score > best_score {
                        best_score = score;
                        best_config = config.clone();
                    }

                    i += 1;
                    if i >= max_iter {
                        break;
                    }
                }

                best_config
            }
            pub fn grid_search(max_iter: Option<usize>) -> #struct_name {
                let config = #struct_name {
                    #( #names3: #types3::default() ),*
                };
                unimplemented!()
            }
        }

    });
    println!("\n{}\n", result);
    result
}
