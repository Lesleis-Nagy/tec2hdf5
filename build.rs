/*
 * To compile third-party non-rust code we use a build.rs script.
 */
fn main() {

    // The 'process_root' function processes the 'src' directory and converts all lalrpop files in
    // to 'rs' files and saves them to OUT_DIR.
    lalrpop::process_root().unwrap();

}
