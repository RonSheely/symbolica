use std::io::{self, BufRead, Write};

use smartstring::{LazyCompact, SmartString};
use symbolica::{
    parser::Token,
    printer::{PrintOptions, RationalPolynomialPrinter},
    rings::{
        integer::IntegerRing, rational::RationalField, rational_polynomial::RationalPolynomial,
    },
    state::{State, Workspace},
};

fn main() {
    let mut buffer = String::with_capacity(2048);
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    // read the number of variables
    let _ = stdin.read_line(&mut buffer).unwrap();
    let mut num_vars_and_var_names = buffer.split(' ');
    let num_vars = num_vars_and_var_names
        .next()
        .expect("Expected number of variables")
        .parse::<usize>()
        .expect("Number of vars should be a non-negative integer");

    let mut var_names: Vec<SmartString<LazyCompact>> = vec![];
    for _ in 0..num_vars {
        var_names.push(
            num_vars_and_var_names
                .next()
                .expect("Expected variable")
                .trim_end()
                .into(),
        );
    }

    let mut state = State::new();
    let workspace = Workspace::default();
    let vars: Vec<_> = var_names
        .iter()
        .map(|v| state.get_or_insert_var(v))
        .collect();

    let print_opt = PrintOptions {
        terms_on_new_line: false,
        color_top_level_sum: false,
        color_builtin_functions: false,
        print_finite_field: false,
        explicit_rational_polynomial: false,
        number_thousands_separator: None,
        multiplication_operator: '*',
        square_brackets_for_function: false,
        num_exp_as_superscript: false,
        latex: false,
    };

    buffer.clear();
    while let Ok(n) = stdin.read_line(&mut buffer) {
        if n == 0 || buffer.starts_with('\n') || buffer.starts_with("&q") {
            break;
        }

        let r: RationalPolynomial<IntegerRing, u16> = Token::parse(&buffer)
            .unwrap()
            .to_rational_polynomial(
                &workspace,
                &mut state,
                RationalField::new(),
                IntegerRing::new(),
                &vars,
                &var_names,
            )
            .unwrap();

        let out_str = format!(
            "{}",
            RationalPolynomialPrinter {
                poly: &r,
                state: &state,
                opts: print_opt
            }
        );

        writeln!(&mut stdout, "{}", out_str).unwrap();

        buffer.clear();
    }
}
