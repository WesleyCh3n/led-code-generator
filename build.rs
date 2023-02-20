fn main() {
    cc::Build::new()
        .file("./c/algorithm.c")
        .compile("alogrithm");
}
