use std::fs;
use serde_json::{Value, json};
use anyhow::Result;

fn convert_openapi_to_jsonschema(input_path: &str, output_path: &str) -> Result<()> {
    let shared_schemas = {
        let yaml_content = fs::read_to_string("../openapi-definitions/schemas.yaml")?;
        let openapi: Value = serde_yaml::from_str(&yaml_content)?;
        openapi.pointer("/components/schemas")
            .cloned()
            .unwrap_or_else(|| json!({}))
    };

    let yaml_content = fs::read_to_string(input_path)?;
    let mut openapi: Value = serde_yaml::from_str(&yaml_content)?;

    if let Some(Value::Object(main_schemas)) = openapi.pointer_mut("/components/schemas") {
        if let Value::Object(shared_map) = shared_schemas {
            for (key, value) in shared_map {
                main_schemas.insert(key, value);
            }
        }
    }

    // Create JSON Schema structure
    let mut jsonschema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$defs": openapi.pointer("/components/schemas").cloned().unwrap_or(json!({}))
    });

    // Fix $ref paths (OpenAPI -> JSON Schema and external refs)
    fn fix_refs(value: &mut Value) {
        match value {
            Value::Object(map) => {
                if let Some(Value::String(ref mut s)) = map.get_mut("$ref") {
                    // Replace OpenAPI references
                    *s = s.replace("#/components/schemas/", "#/$defs/");
                    // Replace external schema.yaml references
                    *s = s.replace("./schemas.yaml#/$defs/", "#/$defs/");
                }
                for v in map.values_mut() {
                    fix_refs(v);
                }
            }
            Value::Array(arr) => {
                for v in arr.iter_mut() {
                    fix_refs(v);
                }
            }
            _ => {}
        }
    }

    fix_refs(&mut jsonschema);

    let output = serde_json::to_string_pretty(&jsonschema)?;
    fs::write(output_path, output)?;

    Ok(())
}
fn main() -> Result<()> {
    convert_openapi_to_jsonschema(
        "../openapi-definitions/auth-server.yaml",
        "../op-types/generated/auth-server-schema.json",
    )?;

    convert_openapi_to_jsonschema(
        "../openapi-definitions/resource-server.yaml",
        "../op-types/generated/resource-server-schema.json",
    )?;
    convert_openapi_to_jsonschema(
        "../openapi-definitions/wallet-address-server.yaml",
        "../op-types/generated/wallet-address-server-schema.json",
    )?;

    Ok(())
}