#![feature(test)]

extern crate test;

use test::{Bencher, black_box};
use rsc::{tokenize, parse};

const SHORT_STR: &str = "5.324 * 54(pad)";
const LONG_STR: &str = "0.999998543 / sqrt(54 ^ (x(3)) % applesauce + bees";
const FUNCTIONS_VARS: &str = "abs(5) + x(3) + abs(x(2)) + sqrt(4)";

macro_rules! tokenizer_bench {
    ($name:ident, $input:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                tokenize(black_box($input))
            });
        }
    };
}

macro_rules! parser_bench {
    ($name:ident, $input:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let tokens = tokenize($input).unwrap();
            b.iter(|| {
                parse(black_box(&tokens))
            })
        }
    };
}

tokenizer_bench!(tokenizer_short_expr, SHORT_STR);
tokenizer_bench!(tokenizer_long_expr, LONG_STR);

parser_bench!(parser_short_expr, SHORT_STR);
parser_bench!(parser_long_expr, LONG_STR);
parser_bench!(parser_function_vars, FUNCTIONS_VARS);
