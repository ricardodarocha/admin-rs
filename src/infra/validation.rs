use serde_json::json;
use serde_json::Value;
use validator::ValidationErrors;

pub fn export_validations(errors: &ValidationErrors, mensagem: &str) -> Value {
        let mut error_map = serde_json::Map::new();
        
        for (field, error) in errors.field_errors() {
            let messages: Vec<String> = error.iter().map(|e| e.message.clone().unwrap_or_default().to_string()).collect();
            error_map.insert(field.to_string(), json!(messages.join(", ")));
        }

        json!({
            "form": error_map,
            "toast": mensagem
        })
    }