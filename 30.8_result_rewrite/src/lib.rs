#![allow(dead_code)]
/*
In this exercise we’re revisiting the expression evaluator exercise
Our initial solution ignores a possible error case: Dividing by zero!

Rewrite eval to instead use idiomatic error handling to handle this error case and return an error when it occurs.

We provide a simple DivideByZeroError type to use as the error type for eval.
*/

/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
	Add,
	Sub,
	Mul,
	Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
	/// An operation on two subexpressions.
	Op {
		op: Operation,
		left: Box<Expression>,
		right: Box<Expression>,
	},

	/// A literal value
	Value(i64),
}

#[derive(PartialEq, Eq, Debug)]
struct DivideByZeroError;

// The original implementation of the expression evaluator. Update this to
// return a `Result` and produce an error when dividing by 0.
fn eval(e: Expression) -> Result<i64, DivideByZeroError> {
	match e {
		Expression::Op { op, left, right } => {
			let left = eval(*left)?;
			let right = eval(*right)?;
			let more_verbose = match op {
				Operation::Add => Ok(left + right),
				Operation::Sub => Ok(left - right),
				Operation::Mul => Ok(left * right),
				Operation::Div => {
					if right != 0 {
						Ok(left / right)
					} else {
						Err(DivideByZeroError)
					}
				}
			};
			let shorter = Ok(match op {
				Operation::Add => left + right,
				Operation::Sub => left - right,
				Operation::Mul => left * right,
				Operation::Div => {
					if right != 0 {
						left / right
					} else {
						return Err(DivideByZeroError);
					}
				}
			});
			assert_eq!(more_verbose, shorter);
			more_verbose
		}
		Expression::Value(v) => Ok(v),
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_error() {
		assert_eq!(
			eval(Expression::Op {
				op: Operation::Div,
				left: Box::new(Expression::Value(99)),
				right: Box::new(Expression::Value(0)),
			}),
			Err(DivideByZeroError)
		);
	}

	#[test]
	fn test_ok() {
		let expr = Expression::Op {
			op: Operation::Sub,
			left: Box::new(Expression::Value(20)),
			right: Box::new(Expression::Value(10)),
		};
		assert_eq!(eval(expr), Ok(10));
	}
}
