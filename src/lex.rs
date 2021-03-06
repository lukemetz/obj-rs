use std::old_io::IoError;
use error::ParseError;

pub fn lex<T, F>(input: &mut T, mut callback: F) -> Option<IoError>
    where T: Buffer, F: FnMut(&str, &[&str]) -> Option<ParseError>
{
    for maybe_line in input.lines() {
        match maybe_line {
            Ok(line) => {
                let line = &line.split('#').next().unwrap();

                let mut words = line.words();
                match words.next() {
                    Some(stmt) => {
                        let args: Vec<&str> = words.collect();
                        let ret = callback(stmt, &args[]);

                        if ret.is_some() {
                            unimplemented!()
                        }
                    }
                    None => {}
                }
            }
            Err(e) => { return Some(e); }
        }
    }
    None
}

#[test]
fn test_lex() {
    use error::{parse_error, ParseErrorKind};

    let input = r#"
   statement0      arg0  arg1	arg2#argX   argX
statement1 arg0    arg1
# Comment
statement2 Hello, world!
"#;

    lex(&mut input.as_bytes(), |stmt, args| {
        match stmt {
            "statement0" => assert_eq!(args, ["arg0", "arg1", "arg2"]),
            "statement1" => assert_eq!(args, ["arg0", "arg1"]),
            "statement2" => assert_eq!(args, ["Hello,", "world!"]),
            _ => return Some(parse_error(ParseErrorKind::UnexpectedStatement))
        }
        None
    });
}

#[cfg(test)]
mod bench {
    //! There is a slight overhead (~30ns) in `lex()` function because it passes arguments as a
    //! slice not an iterator.

    extern crate test;

    #[bench]
    fn pass_slice(b: &mut test::Bencher) {
        b.iter(|| {
            let words = "1.00 2.00 3.00".words();
            let args: Vec<&str> = words.collect();
            let args = &args[];

            args.iter().map(|&input| input.parse::<f32>().unwrap()).collect::<Vec<f32>>();
        })
    }

    #[bench]
    fn pass_iter(b: &mut test::Bencher) {
        b.iter(|| {
            let words = "1.00 2.00 3.00".words();

            words.map(|input| input.parse::<f32>().unwrap()).collect::<Vec<f32>>();
        })
    }
}
