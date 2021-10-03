use untitledlang::{interpret_, run_repl};

fn main() {
    let source = r#"
    fun foo() {
        print 10;
    }
    foo();
    "#;
    interpret_(source);
    // run_repl();
}
