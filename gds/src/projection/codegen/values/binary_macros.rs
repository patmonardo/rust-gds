/// Generate a GdsValue implementation for a binary value with MIME type
#[macro_export]
macro_rules! gds_value_binary {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name {
            data: Vec<u8>,
            mime_type: Option<String>,
        }

        impl $name {
            pub fn new(data: Vec<u8>, mime_type: Option<String>) -> Self {
                Self { data, mime_type }
            }
        }

        impl $crate::values::traits::GdsValue for $name {
            fn value_type(&self) -> $crate::types::ValueType {
                $crate::types::ValueType::ByteArray
            }
            fn as_object(&self) -> serde_json::Value {
                let mut obj = serde_json::Map::new();
                obj.insert("data".to_string(), serde_json::Value::from(self.data.clone()));
                if let Some(mime) = &self.mime_type {
                    obj.insert("mime_type".to_string(), serde_json::Value::from(mime.clone()));
                }
                serde_json::Value::Object(obj)
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        // Implement BinaryValue trait methods directly
        impl $name {
            pub fn binary_data(&self) -> &[u8] {
                &self.data
            }
            pub fn mime_type(&self) -> Option<&str> {
                self.mime_type.as_deref()
            }
        }
    };
}
