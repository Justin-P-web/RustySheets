#[derive(Debug, Clone)]
pub enum EvalError {
    DivisionByZero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation{
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Expression 
{
    Literal(Literal),
    Operation {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Formula(Operation, Vec<Expression>),

}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    // Canonical form: den > 0, gcd(|num|, den) == 1
    pub num: i64,
    pub den: i64,
}

impl Rational {
    pub fn new(num: i64, den: i64) -> Result<Self, EvalError> {
        if den == 0 { return Err(EvalError::DivisionByZero); }
        if num == 0 { return Ok(Self { num: 0, den: 1 }); }

        let mut n = num;
        let mut d = den;

        if d < 0 { n = -n; d = -d; }

        let g = gcd_i64(n.abs(), d);
        Ok(Self { num: n / g, den: d / g })
    }

    pub fn from_int(x: i64) -> Self { Self { num: x, den: 1 } }
}

fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs().max(1)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal{
    Int(i64),
    Rational(Rational),
    Float(f64),
}
