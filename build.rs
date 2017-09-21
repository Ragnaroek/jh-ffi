extern crate gcc;

fn main() {
    gcc::Build::new()
        .file("ext/jh/c_jh.c")
        .compile("libjh.a");
}
