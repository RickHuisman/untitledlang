use std::io;
use std::borrow::BorrowMut;
use crate::vm::vm::interpret;

mod lexer;
mod parser;
mod compiler;
mod vm;

pub fn run_repl() {
    loop {
        interpret(&read_line());
    }
}

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            trim_newline(input.borrow_mut());
            input
        }
        Err(error) => {
            panic!("error: {}", error);
        }
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::lex;
    use crate::parser::parser::parse;
    use crate::compiler::compiler::compile;
    use crate::vm::vm::VM;
    use std::io::Cursor;
    use crate::compiler::object::Closure;
    use crate::vm::obj::Gc;
    use crate::compiler::value::Value;
    use regex::Regex;
    use std::fs;

    fn parse_expects(source: &str, regex: Regex, field: usize) -> Vec<String> {
        let mut results = vec![];
        for line in source.lines() {
            let caps = regex.captures(line);
            if let Some(caps) = caps {
                results.push(caps[field].to_owned());
            }
        }

        results
    }

    #[derive(PartialEq, Debug)]
    enum TestResult {
        Ok,
        CompileError,
        RuntimeError,
    }

    fn execute(source: &str) -> (Vec<String>, TestResult) {
        let fun = compile(source).unwrap();

        let mut output = vec![];
        let cursor = Cursor::new(&mut output);

        let mut vm = VM::with_stdout(cursor);
        let result = match vm.interpret(fun) {
            Ok(_) => TestResult::Ok,
            Err(err) => {
                println!("Runtime error: {:?}", err);
                TestResult::RuntimeError
            }
        };

        let output = String::from_utf8(output).unwrap();

        (output.lines().map(|l| l.to_owned()).collect(), result)
    }

    fn harness(source: &str) {
        let expects = parse_expects(
            source,
            Regex::new(r"// expect: ?(.*)").unwrap(),
            1,
        );

        let expected_result =
            if !parse_expects(source, Regex::new(r"\[line (\d+)\] (Error.+)").unwrap(), 2).is_empty() {
                TestResult::CompileError
            } else if !parse_expects(source, Regex::new(r"// (Error.*)").unwrap(), 1).is_empty() {
                TestResult::CompileError
            } else if !parse_expects(
                source,
                Regex::new(r"// expect runtime error: (.+)").unwrap(),
                1,
            ).is_empty()
            {
                TestResult::RuntimeError
            } else {
                TestResult::Ok
            };

        let (output, result) = execute(source);
        assert_eq!(expects, output);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn run_tests() {
        // Runs every test in the test folder.
        let paths = fs::read_dir("./test").unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let source = fs::read_to_string(path.to_str().unwrap()).unwrap();
            harness(&source);
        }
    }
}