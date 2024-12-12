use std::env;
use std::path::PathBuf;
use cc::{self, Tool};
fn main(){
    //let tool = cc::Tool::new();
    let target = std::env::var("TARGET").unwrap();
    if let Some(tool) = cc::windows_registry::find_tool(&target.as_str(), "cl.exe") {
        for (key, value) in tool.env() {
            std::env::set_var(key, value);
        }
    }
    cc::Build::new().file("src/lib.c").file("src/acc/acc.c").file("src/raceroom/raceroom.c").cargo_output(false).compile("lib_rrl");
    let bindings = bindgen::Builder::default()
    .header("src/lib.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}