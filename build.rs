extern crate cc;

fn main() {
    cc::Build::new()
        .file("ext/jh/c_jh.c")
        .compile("libjh.a");
}
