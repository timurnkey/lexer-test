fn main() {
    lalrpop::Configuration::new()
        .set_in_dir("src/grammar")
        .set_out_dir("src/gen")
        .process()
        .expect("it works")
}
