use std::fmt::{Display, Formatter, Result};

use rlua::Value;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ValueType {
    /// The Lua value `nil`.
    Nil,
    /// The Lua value `true` or `false`.
    Boolean,
    /// A "light userdata" object, equivalent to a raw pointer.
    LightUserData,
    /// An integer number.
    ///
    /// Any Lua number convertible to a `Integer` will be represented as this variant.
    Integer,
    /// A floating point number.
    Number,
    /// An interned string, managed by Lua.
    ///
    /// Unlike Rust strings, Lua strings may not be valid UTF-8.
    String,
    /// Reference to a Lua table.
    Table,
    /// Reference to a Lua function (or closure).
    Function,
    /// Reference to a Lua thread (or coroutine).
    Thread,
    /// Reference to a userdata object that holds a custom type which implements `UserData`.
    /// Special builtin userdata types will be represented as other `Value` variants.
    UserData,
    /// `Error` is a special builtin userdata type.  When received from Lua it is implicitly cloned.
    Error,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ValueType::*;

        match self {
            Nil => write!(f, "Nil"),
            Boolean => write!(f, "Boolean"),
            LightUserData => write!(f, "LightUserData"),
            Integer => write!(f, "Integer"),
            Number => write!(f, "Number"),
            String => write!(f, "String"),
            Table => write!(f, "Table"),
            Function => write!(f, "Function"),
            Thread => write!(f, "Thread"),
            UserData => write!(f, "UserData"),
            Error => write!(f, "Error"),
        }
    }
}

pub fn value_type(value: &Value) -> ValueType {
    match value {
        Value::Nil => ValueType::Nil,
        Value::Boolean(_) => ValueType::Boolean,
        Value::LightUserData(_) => ValueType::LightUserData,
        Value::Integer(_) => ValueType::Integer,
        Value::Number(_) => ValueType::Number,
        Value::String(_) => ValueType::String,
        Value::Table(_) => ValueType::Table,
        Value::Function(_) => ValueType::Function,
        Value::Thread(_) => ValueType::Thread,
        Value::UserData(_) => ValueType::UserData,
        Value::Error(_) => ValueType::Error,
    }
}
