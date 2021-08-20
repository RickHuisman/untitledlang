use crate::vm::interpret;
use std::io;

mod compiler;
mod lexer;
mod parser;
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
            trim_newline(&mut input);
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
    use crate::vm::interpret_with_stdout;
    use regex::Regex;
    use std::fs;
    use std::io::Cursor;
    use walkdir::WalkDir;

    #[derive(PartialEq, Debug)]
    enum TestResult {
        Ok,
        CompileError,
        RuntimeError,
    }

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

    fn extract_expects(source: &str) -> TestResult {
        if !parse_expects(source, Regex::new(r"\[line (\d+)\] (Error.+)").unwrap(), 2).is_empty() {
            return TestResult::CompileError;
        }

        if !parse_expects(source, Regex::new(r"// (Error.*)").unwrap(), 1).is_empty() {
            return TestResult::CompileError;
        }

        if !parse_expects(
            source,
            Regex::new(r"// expect runtime error: (.+)").unwrap(),
            1,
        )
        .is_empty()
        {
            return TestResult::RuntimeError;
        }

        TestResult::Ok
    }

    fn execute(source: &str) -> (Vec<String>, TestResult) {
        let mut output = vec![];
        let cursor = Cursor::new(&mut output);

        let result = match interpret_with_stdout(source, cursor) {
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
        let expects = parse_expects(source, Regex::new(r"// expect: ?(.*)").unwrap(), 1);

        let expected_result = extract_expects(source);

        let (output, result) = execute(source);
        assert_eq!(expects, output);
        assert_eq!(expected_result, result);
    }

    fn run_test_file(path: String) {
        let source = fs::read_to_string(path).unwrap();
        harness(&source);
    }

    #[test]
    fn run_tests() {
        // Runs every test in the test folder.
        for f in WalkDir::new("./test").into_iter().filter_map(|e| e.ok()) {
            if f.metadata().unwrap().is_file() {
                let path = f.path().display().to_string();
                run_test_file(path);
            }
        }
    }
}
