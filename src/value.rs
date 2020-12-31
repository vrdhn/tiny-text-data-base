use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    PrimaryKey { key: Uuid },
    Foreignkey { table_name: String, key: Uuid },
    Integer { value: i64 },
    String { value: String },
    Epoch { epoch: u64 },
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn is_primary_key(&self) -> bool {
        matches!( self, Value::PrimaryKey{key:_})
    }

    pub fn string_equals(&self, val: &str) -> bool {
        match self {
            Value::String { value: v } => v == val,
            _ => false,
        }
    }

    pub fn new_string(value: &str) -> Value {
        Value::String {
            value: value.to_string(),
        }
    }
    pub fn new_integer(value: i64) -> Value {
        Value::Integer { value: value }
    }

    pub fn as_int(&self) -> Option<i64> {
        if let Value::Integer { value } = self {
            Some(*value)
        } else {
            None
        }
    }
    pub fn as_string(&self) -> Option<String> {
        if let Value::String { value } = self {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn as_pkey(&self) -> Option<Uuid> {
        if let Value::PrimaryKey { key } = self {
            Some(*key)
        } else {
            None
        }
    }
}
