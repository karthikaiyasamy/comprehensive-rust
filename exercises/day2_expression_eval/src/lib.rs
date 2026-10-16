#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Value(i64),
    Op {
        op: Op,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

pub fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Value(val) => Ok(val),
        Expression::Op { op, left, right } => {
            let l = eval(*left)?;
            let r = eval(*right)?;
            match op {
                Op::Add => Ok(l + r),
                Op::Sub => Ok(l - r),
                Op::Mul => Ok(l * r),
                Op::Div => {
                    if r == 0 {
                        Err(String::from("division by zero"))
                    } else {
                        Ok(l / r)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let expr = Expression::Op {
            op: Op::Sub,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Op {
                op: Op::Mul,
                left: Box::new(Expression::Value(2)),
                right: Box::new(Expression::Value(3)),
            }),
        };
        assert_eq!(eval(expr), Ok(4));
    }

    #[test]
    fn test_div_by_zero() {
        let div_zero = Expression::Op {
            op: Op::Div,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(0)),
        };
        assert_eq!(eval(div_zero), Err(String::from("division by zero")));
    }
}
