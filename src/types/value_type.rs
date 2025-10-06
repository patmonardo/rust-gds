use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the data types that can be used for properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValueType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Boolean,
    Char,
    String,
    BigInt,
    Decimal,
    Date,
    DateTime,
    Null,
    ByteArray,
    ShortArray,
    IntArray,
    LongArray,
    FloatArray,
    DoubleArray,
    BooleanArray,
    CharArray,
    StringArray,
    BigIntArray,
    DecimalArray,
    DateArray,
    DateTimeArray,
    UntypedArray,
    Unknown,
    // Map types for key-value structures
    StringMap,
    LongMap,
    DoubleMap,
    BooleanMap,
    StringMapArray,
    LongMapArray,
    DoubleMapArray,
    BooleanMapArray,
    UntypedMap,
}

impl ValueType {
    pub fn name(self) -> &'static str {
        match self {
            ValueType::Byte => "BYTE",
            ValueType::Short => "SHORT",
            ValueType::Int => "INT",
            ValueType::Long => "LONG",
            ValueType::Float => "FLOAT",
            ValueType::Double => "DOUBLE",
            ValueType::Boolean => "BOOLEAN",
            ValueType::Char => "CHAR",
            ValueType::String => "STRING",
            ValueType::BigInt => "BIGINT",
            ValueType::Decimal => "DECIMAL",
            ValueType::Date => "DATE",
            ValueType::DateTime => "DATETIME",
            ValueType::Null => "NULL",
            ValueType::ByteArray => "BYTE_ARRAY",
            ValueType::ShortArray => "SHORT_ARRAY",
            ValueType::IntArray => "INT_ARRAY",
            ValueType::LongArray => "LONG_ARRAY",
            ValueType::FloatArray => "FLOAT_ARRAY",
            ValueType::DoubleArray => "DOUBLE_ARRAY",
            ValueType::BooleanArray => "BOOLEAN_ARRAY",
            ValueType::CharArray => "CHAR_ARRAY",
            ValueType::StringArray => "STRING_ARRAY",
            ValueType::BigIntArray => "BIGINT_ARRAY",
            ValueType::DecimalArray => "DECIMAL_ARRAY",
            ValueType::DateArray => "DATE_ARRAY",
            ValueType::DateTimeArray => "DATETIME_ARRAY",
            ValueType::UntypedArray => "UNTYPED_ARRAY",
            ValueType::Unknown => "UNKNOWN",
            ValueType::StringMap => "STRING_MAP",
            ValueType::LongMap => "LONG_MAP",
            ValueType::DoubleMap => "DOUBLE_MAP",
            ValueType::BooleanMap => "BOOLEAN_MAP",
            ValueType::StringMapArray => "STRING_MAP_ARRAY",
            ValueType::LongMapArray => "LONG_MAP_ARRAY",
            ValueType::DoubleMapArray => "DOUBLE_MAP_ARRAY",
            ValueType::BooleanMapArray => "BOOLEAN_MAP_ARRAY",
            ValueType::UntypedMap => "UNTYPED_MAP",
        }
    }

    pub fn csv_name(self) -> Result<&'static str, &'static str> {
        match self {
            ValueType::Byte => Ok("byte"),
            ValueType::Short => Ok("short"),
            ValueType::Int => Ok("int"),
            ValueType::Long => Ok("long"),
            ValueType::Float => Ok("float"),
            ValueType::Double => Ok("double"),
            ValueType::Boolean => Ok("boolean"),
            ValueType::Char => Ok("char"),
            ValueType::String => Ok("string"),
            ValueType::BigInt => Ok("bigint"),
            ValueType::Decimal => Ok("decimal"),
            ValueType::Date => Ok("date"),
            ValueType::DateTime => Ok("datetime"),
            ValueType::Null => Ok("null"),
            ValueType::ByteArray => Ok("byte[]"),
            ValueType::ShortArray => Ok("short[]"),
            ValueType::IntArray => Ok("int[]"),
            ValueType::LongArray => Ok("long[]"),
            ValueType::FloatArray => Ok("float[]"),
            ValueType::DoubleArray => Ok("double[]"),
            ValueType::BooleanArray => Ok("boolean[]"),
            ValueType::CharArray => Ok("char[]"),
            ValueType::StringArray => Ok("string[]"),
            ValueType::BigIntArray => Ok("bigint[]"),
            ValueType::DecimalArray => Ok("decimal[]"),
            ValueType::DateArray => Ok("date[]"),
            ValueType::DateTimeArray => Ok("datetime[]"),
            ValueType::UntypedArray => Ok("Any[]"),
            ValueType::Unknown => Err("ValueType::UNKNOWN has no CSV name"), // Changed
            ValueType::StringMap => Ok("string{}"),                          // Added
            ValueType::LongMap => Ok("long{}"),                              // Added
            ValueType::DoubleMap => Ok("double{}"),
            ValueType::BooleanMap => Ok("boolean{}"),
            ValueType::StringMapArray => Ok("string{}[]"),
            ValueType::LongMapArray => Ok("long{}[]"),
            ValueType::DoubleMapArray => Ok("double{}[]"),
            ValueType::BooleanMapArray => Ok("boolean{}[]"),
            ValueType::UntypedMap => Ok("Any{}"),
        }
    }

    pub fn cypher_name(self) -> &'static str {
        match self {
            ValueType::Byte => "Byte",
            ValueType::Short => "Short",
            ValueType::Int => "Integer",
            ValueType::Long => "Long",
            ValueType::Float => "Float",
            ValueType::Double => "Double",
            ValueType::Boolean => "Boolean",
            ValueType::Char => "Char",
            ValueType::String => "String",
            ValueType::BigInt => "BigInt",
            ValueType::Decimal => "Decimal",
            ValueType::Date => "Date",
            ValueType::DateTime => "DateTime",
            ValueType::Null => "Null",
            ValueType::ByteArray => "List of Byte",
            ValueType::ShortArray => "List of Short",
            ValueType::IntArray => "List of Integer",
            ValueType::LongArray => "List of Long",
            ValueType::FloatArray => "List of Float",
            ValueType::DoubleArray => "List of Double",
            ValueType::BooleanArray => "List of Boolean",
            ValueType::CharArray => "List of Char",
            ValueType::StringArray => "List of String",
            ValueType::BigIntArray => "List of BigInt",
            ValueType::DecimalArray => "List of Decimal",
            ValueType::DateArray => "List of Date",
            ValueType::DateTimeArray => "List of DateTime",
            ValueType::UntypedArray => "List of Any",
            ValueType::Unknown => "Unknown",
            ValueType::StringMap => "Map of String",
            ValueType::LongMap => "Map of Long",
            ValueType::DoubleMap => "Map of Double",
            ValueType::BooleanMap => "Map of Boolean",
            ValueType::StringMapArray => "List of Map of String",
            ValueType::LongMapArray => "List of Map of Long",
            ValueType::DoubleMapArray => "List of Map of Double",
            ValueType::BooleanMapArray => "List of Map of Boolean",
            ValueType::UntypedMap => "Map of Any",
        }
    }

    pub fn is_compatible_with(self, other: ValueType) -> bool {
        if self == other {
            return true;
        }

        if other == ValueType::UntypedArray {
            matches!(
                self,
                ValueType::LongArray
                    | ValueType::FloatArray
                    | ValueType::DoubleArray
                    | ValueType::BooleanArray
                    | ValueType::StringArray
                    | ValueType::BigIntArray
                    | ValueType::UntypedArray
            )
        } else if other == ValueType::UntypedMap {
            matches!(
                self,
                ValueType::StringMap
                    | ValueType::LongMap
                    | ValueType::DoubleMap
                    | ValueType::BooleanMap
                    | ValueType::UntypedMap
            )
        } else {
            (self == ValueType::Float && other == ValueType::Double)
                || (self == ValueType::Long && other == ValueType::BigInt)
        }
    }
}

/// Visitor trait for ValueType. Optional visitor methods return Option<R>.
pub trait Visitor<R> {
    fn visit_byte(&self) -> R;
    fn visit_short(&self) -> R;
    fn visit_int(&self) -> R;
    fn visit_long(&self) -> R;
    fn visit_float(&self) -> R;
    fn visit_double(&self) -> R;
    fn visit_boolean(&self) -> R;
    fn visit_char(&self) -> R;
    fn visit_string(&self) -> R;
    fn visit_bigint(&self) -> R;
    fn visit_decimal(&self) -> R;
    fn visit_date(&self) -> R;
    fn visit_datetime(&self) -> R;
    fn visit_null(&self) -> R;
    fn visit_byte_array(&self) -> R;
    fn visit_short_array(&self) -> R;
    fn visit_int_array(&self) -> R;
    fn visit_long_array(&self) -> R;
    fn visit_float_array(&self) -> R;
    fn visit_double_array(&self) -> R;
    fn visit_boolean_array(&self) -> R;
    fn visit_char_array(&self) -> R;
    fn visit_string_array(&self) -> R;
    fn visit_bigint_array(&self) -> R;
    fn visit_decimal_array(&self) -> R;
    fn visit_date_array(&self) -> R;
    fn visit_datetime_array(&self) -> R;
    fn visit_untyped_array(&self) -> Option<R> {
        None
    }
    fn visit_unknown(&self) -> Option<R> {
        None
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
