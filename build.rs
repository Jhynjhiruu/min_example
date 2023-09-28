fn main() {
    cc::Build::new().file("src/build1.c").compile("build1");
    cc::Build::new().file("src/build2.c").compile("build2");
}
