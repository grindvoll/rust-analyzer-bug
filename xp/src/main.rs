use xp_derive::LitIntBug;

#[derive(LitIntBug)]
struct Sample;


fn main() {
    // This print statement will print the Debug representation of syn::parse_str::<syn::LitInt>("-12").
    // Note that the compiler and RustAnalyzer parses this differently! This can be verified
    // by running this function and inspect the output. Compare this with the tooltip offered
    // in VSCode by hovering over the `LIT_INT_FROM_STR` constant.
    println!("{}", INT_DEBUG_STR);

    let a = vec![-1 , -2];

}
