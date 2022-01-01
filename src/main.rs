use pom::{parser::*, Parser};
use rand::Rng;
use std::str::{self, FromStr};

fn space() -> Parser<u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn number() -> Parser<u8, f64> {
    let integer = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0');
    let frac = sym(b'.') + one_of(b"0123456789").repeat(1..);
    let exp = one_of(b"eE") + one_of(b"+-").opt() + one_of(b"0123456789").repeat(1..);
    let number = sym(b'-').opt() + integer + frac.opt() + exp.opt();
    number
        .collect()
        .convert(str::from_utf8)
        .convert(f64::from_str)
}

fn expr() -> Parser<u8, f64> {
    let sub = sym(b'(')
        * (call(add) | call(sub) | call(mul) | call(div) | call(max) | call(min))
        - sym(b')');
    space() * (number() | sub | d20()) - space()
}

fn add() -> Parser<u8, f64> {
    let add_expr = seq(b"add") * (expr() + expr());
    add_expr.map(|(a, b)| a + b)
}

fn sub() -> Parser<u8, f64> {
    let sub_expr = seq(b"sub") * (expr() + expr());
    sub_expr.map(|(a, b)| a - b)
}

fn mul() -> Parser<u8, f64> {
    let mul_expr = seq(b"mul") * (expr() + expr());
    mul_expr.map(|(a, b)| a * b)
}

fn div() -> Parser<u8, f64> {
    let div_expr = seq(b"div") * (expr() + expr());
    div_expr.map(|(a, b)| a / b)
}

fn max() -> Parser<u8, f64> {
    let max_expr = seq(b"max") * list(expr(), space());
    max_expr.map(|args| {
        args.into_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    })
}

fn min() -> Parser<u8, f64> {
    let min_expr = seq(b"min") * list(expr(), space());
    min_expr.map(|args| {
        args.into_iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    })
}

fn d20() -> Parser<u8, f64> {
    let d20_expr = seq(b"d") * number();
    d20_expr.map(|a| {
        let u: u32 = rand::thread_rng().gen_range(0..(a as u32));
        u as f64
    })
}

fn main() {
    println!("{:?}", expr().parse(b"(max d20 d20)"));
    println!("{:?}", expr().parse(b"(max d20 d20)"));
    println!("{:?}", expr().parse(b"(max 9 17.89)"));
    println!(
        "{:?}",
        expr().parse(b"(max (div (add 1 (mul 3 1.5e+1)) 4) 17.89)")
    );
}
