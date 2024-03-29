use inner::inner;

use crate::Raz_vm::error::RazError;
use crate::Raz_vm::value::{ApplyOperatorResult, ConvertResult, Value, ValueType};

/// try to apply an infix operator
pub fn apply_operator(self_: &Value, name: &str, other: &Value) -> ApplyOperatorResult {
    match name {
        "==" => ApplyOperatorResult::Ok(Value::Boolean(self_ == other)),
        "!=" => ApplyOperatorResult::Ok(Value::Boolean(self_ != other)),
        "||" => ApplyOperatorResult::Ok(Value::Boolean(self_.is_truthy() || other.is_truthy())),
        "&&" => ApplyOperatorResult::Ok(Value::Boolean(self_.is_truthy() && other.is_truthy())),
        _ => match self_ {
            Value::Int(i) => match name {
                "+" => {
                    let other_int_try = other.try_convert(ValueType::Int);

                    match other_int_try {
                        ConvertResult::Ok(x) => {
                            ApplyOperatorResult::Ok(Value::Int(i + inner!(x, if Value::Int)))
                        }
                        ConvertResult::NotOk => ApplyOperatorResult::NoSuchOperator,
                        ConvertResult::SameType => {
                            ApplyOperatorResult::Ok(Value::Int(i + inner!(other, if Value::Int)))
                        }
                    }
                }
                "-" => {
                    let other_int_try = other.try_convert(ValueType::Int);

                    match other_int_try {
                        ConvertResult::Ok(x) => {
                            ApplyOperatorResult::Ok(Value::Int(i - inner!(x, if Value::Int)))
                        }
                        ConvertResult::NotOk => ApplyOperatorResult::NoSuchOperator,
                        ConvertResult::SameType => {
                            ApplyOperatorResult::Ok(Value::Int(i - inner!(other, if Value::Int)))
                        }
                    }
                }
                "*" => {
                    let other_int_try = other.try_convert(ValueType::Int);

                    match other_int_try {
                        ConvertResult::Ok(x) => {
                            ApplyOperatorResult::Ok(Value::Int(i * inner!(x, if Value::Int)))
                        }
                        ConvertResult::NotOk => ApplyOperatorResult::NoSuchOperator,
                        ConvertResult::SameType => {
                            ApplyOperatorResult::Ok(Value::Int(i * inner!(other, if Value::Int)))
                        }
                    }
                }
                "/" => {
                    let other_int_try = other.try_convert(ValueType::Int);

                    match other_int_try {
                        ConvertResult::Ok(x) => {
                            let o = inner!(x, if Value::Int);
                            if o == 0 {
                                return ApplyOperatorResult::Error(
                                    RazError::ZeroDivisionOrModulo,
                                );
                            }
                            ApplyOperatorResult::Ok(Value::Int(i / o))
                        }
                        ConvertResult::NotOk => ApplyOperatorResult::NoSuchOperator,
                        ConvertResult::SameType => {
                            let o = *inner!(other, if Value::Int);
                            if o == 0 {
                                return ApplyOperatorResult::Error(
                                    RazError::ZeroDivisionOrModulo,
                                );
                            }
                            ApplyOperatorResult::Ok(Value::Int(i / o))
                        }
                    }
                }
                "%" => {
                    let other_int_try = other.try_convert(ValueType::Int);

                    match other_int_try {
                        ConvertResult::Ok(x) => {
                            let o = inner!(x, if Value::Int);
                            if o == 0 {
                                return ApplyOperatorResult::Error(
                                    RazError::ZeroDivisionOrModulo,
                                );
                            }
                            ApplyOperatorResult::Ok(Value::Int(i % o))
                        }
                        ConvertResult::NotOk => ApplyOperatorResult::NoSuchOperator,
                        ConvertResult::SameType => {
                            let o = *inner!(other, if Value::Int);
                            if o == 0 {
                                return ApplyOperatorResult::Error(
                                    RazError::ZeroDivisionOrModulo,
                                );
                            }
                            ApplyOperatorResult::Ok(Value::Int(i % o))
                        }
                    }
                }

                ">" => {
                    let other_int_try = other.try_convert(ValueType::Int);

                    match other_int_try {
                        ConvertResult::Ok(x) => {
                            ApplyOperatorResult::Ok(Value::Boolean(i > &inner!(x, if Value::Int)))
                        }
                        ConvertResult::NotOk => ApplyOperatorResult::NoSuchOperator,
                        ConvertResult::SameType => ApplyOperatorResult::Ok(Value::Boolean(
                            i > inner!(other, if Value::Int),
                        )),
                    }
                }
                "<" => {
                    let other_int_try = other.try_convert(ValueType::Int);

                    match other_int_try {
                        ConvertResult::Ok(x) => {
                            ApplyOperatorResult::Ok(Value::Boolean(i <= &inner!(x, if Value::Int)))
                        }
                        ConvertResult::NotOk => ApplyOperatorResult::NoSuchOperator,
                        ConvertResult::SameType => ApplyOperatorResult::Ok(Value::Boolean(
                            i <= inner!(other, if Value::Int),
                        )),
                    }
                }
                ">=" => {
                    let other_int_try = other.try_convert(ValueType::Int);

                    match other_int_try {
                        ConvertResult::Ok(x) => {
                            ApplyOperatorResult::Ok(Value::Boolean(i >= &inner!(x, if Value::Int)))
                        }
                        ConvertResult::NotOk => ApplyOperatorResult::NoSuchOperator,
                        ConvertResult::SameType => ApplyOperatorResult::Ok(Value::Boolean(
                            i >= inner!(other, if Value::Int),
                        )),
                    }
                }
                "<=" => {
                    let other_int_try = other.try_convert(ValueType::Int);

                    match other_int_try {
                        ConvertResult::Ok(x) => {
                            ApplyOperatorResult::Ok(Value::Boolean(i <= &inner!(x, if Value::Int)))
                        }
                        ConvertResult::NotOk => ApplyOperatorResult::NoSuchOperator,
                        ConvertResult::SameType => ApplyOperatorResult::Ok(Value::Boolean(
                            i <= inner!(other, if Value::Int),
                        )),
                    }
                }

                _ => ApplyOperatorResult::NoSuchOperator,
            },
            Value::String(s) => match name {
                "+" => {
                    let other_string_try = other.try_convert(ValueType::String);

                    match other_string_try {
                        ConvertResult::Ok(x) => ApplyOperatorResult::Ok(Value::String(
                            s.to_owned() + &*inner!(x, if Value::String),
                        )),
                        ConvertResult::NotOk => ApplyOperatorResult::NoSuchOperator,
                        ConvertResult::SameType => ApplyOperatorResult::Ok(Value::String(
                            s.to_owned() + &*inner!(other, if Value::String),
                        )),
                    }
                }
                _ => ApplyOperatorResult::NoSuchOperator,
            },
            _ => ApplyOperatorResult::NoSuchOperator,
        },
    }
}

pub fn apply_unary_operator(self_: &Value, name: &str) -> ApplyOperatorResult {
    match self_ {
        Value::Int(x) => match name {
            "-" => ApplyOperatorResult::Ok(Value::Int(-*x)),
            "+" => ApplyOperatorResult::Ok(Value::Int(*x)),
            _ => ApplyOperatorResult::NoSuchOperator,
        },
        Value::Boolean(x) => match name {
            "!" => ApplyOperatorResult::Ok(Value::Boolean(!*x)),
            _ => ApplyOperatorResult::NoSuchOperator,
        },
        _ => ApplyOperatorResult::NoSuchOperator,
    }
}
