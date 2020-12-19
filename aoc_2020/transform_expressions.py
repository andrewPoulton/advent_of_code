
import re

replacements = {key:f"MyInt::new({key})" for key in "0123456789"}

def transform(input):
    out = []
    with open(input, 'r') as expressions:
        for expression in expressions.readlines():
            expression = expression.replace("*","@").replace("+","*").replace("@", "+")
            expression = "out += " + expression.strip()
            for key, val in replacements.items():
                expression = expression.replace(key, val)
            out.append(expression+';')
    return "\n".join(out)

boilerplate = f"""
#[derive(Debug)]
struct MyInt{{
    val: i64
}}

impl MyInt{{
    fn new(val:i64)->Self{{
        Self {{ val }}
    }}
}}

impl std::ops::Add for MyInt{{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {{
        Self {{val: self.val * rhs.val}}
    }}
}}

impl std::ops::AddAssign for MyInt{{

    fn add_assign(&mut self, rhs: Self) {{
        self.val += rhs.val;
    }}
}}

impl std::ops::Mul for MyInt{{
    type Output = Self;
    fn mul(self, rhs:Self)->Self::Output {{
        Self {{val: self.val + rhs.val}}
    }}
}}

pub fn calculate()->i64{{
    let mut out: MyInt = MyInt::new(0);
    {transform("src/day18.txt")}
    out.val
}}
"""
open("src/day18_assist.rs", "w").write(boilerplate)