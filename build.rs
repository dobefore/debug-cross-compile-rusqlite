use std::env;
use std::path::Path;

fn main() {
  
    let pat = "tls";
    let key = format!("CARGO_FEATURE_{}", pat).to_uppercase();
    if env::var_os(key).is_some() {
        println!("cargo:rustc-cfg=feature=\"{}\"", pat)
    }
}
