use crate::Raz_vm::value::{Value, ValueType};
use crate::Raz_vm::vm::Heap;

#[derive(Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum ErrorType {
    UndefinedVariable(String),
    InvalidBinaryOperation(Value, String, Value),
    InvalidUnaryOperation(Value, String),
    ZeroDivisionOrModulo,
    NotCallable(ValueType),
    NoProperty(Value, String),
    ArgumentError(String),
    ConversionError(String),
    InFunction(String, Box<ErrorType>),
    Failure(String),
}

impl ErrorType {
    pub fn to_string(&self, heap: &Heap) -> String {
        match self {
            ErrorType::UndefinedVariable(name) => {
                format!("Undefined Variable: {}", name)
            }
            ErrorType::InvalidBinaryOperation(a, o, b) => {
                format!(
                    "Invalid Binary Operation: {} {} {}",
                    a.to_debug_string(heap),
                    o,
                    b.to_debug_string(heap)
                )
            }
            ErrorType::InvalidUnaryOperation(a, o) => {
                format!("Invalid Unary Operation: {}{}", o, a.to_debug_string(heap))
            }
            ErrorType::ZeroDivisionOrModulo => {
                format!("Division or Modulo by Zero")
            }
            ErrorType::NotCallable(t) => {
                format!("Type {:?} is not callable", t)
            }
            ErrorType::NoProperty(val, name) => {
                format!(
                    "Property '{}' does not exist on {}",
                    name,
                    val.to_debug_string(heap)
                )
            }
            ErrorType::ArgumentError(x) | ErrorType::ConversionError(x) | ErrorType::Failure(x) => {
                x.clone()
            }
            ErrorType::InFunction(x, y) => {
                format!("In Function {}:\n{}", x, y.to_string(heap))
            }
        }
    }
}

pub type RazError = ErrorType;
