use crate::error::LoxError;
use crate::interpreter::Interpreter;
use crate::object::Object;

pub trait Callable: Clone + Sized {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Object])
        -> Result<Object, LoxError>;
}
