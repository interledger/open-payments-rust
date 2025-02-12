use serde_json::{json, Value};
use anyhow::Result;

pub fn convert_openapi_to_jsonschema_with_shared_schema(
    io_paths: &[(&str, &str)],
    shared_schema_path: &str,
) -> Result<()> {
    println!("{}", shared_schema_path);
    let shared_yaml_content = std::fs::read_to_string(&shared_schema_path)?;
    println!("{}", shared_yaml_content);

    let shared_yaml: Value = serde_yaml::from_str(&shared_yaml_content)?;
    let shared_schemas = shared_yaml
        .pointer("/components/schemas")
        .cloned()
        .unwrap_or_else(|| json!({}));
    println!("{}", shared_schema_path);

    io_paths.iter().try_for_each(|(input_path, output_path)| -> Result<()> {
        let input_yaml_content = std::fs::read_to_string(input_path)?;
        let mut openapi: Value = serde_yaml::from_str(&input_yaml_content)?;

        if let Some(Value::Object(main_schemas)) = openapi.pointer_mut("/components/schemas") {
            if let Value::Object(shared_map) = &shared_schemas {
                for (key, value) in shared_map {
                    main_schemas.insert(key.clone(), value.clone());
                }
            }
        }

        // Create JSONSchema structure
        let mut jsonschema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": openapi.pointer("/components/schemas")
                         .cloned()
                         .unwrap_or(json!({}))
        });

        
        fix_refs(&mut jsonschema);

        let output_str = serde_json::to_string_pretty(&jsonschema)?;
        std::fs::write(output_path, output_str)?;

        Ok(())
    })
}

// Fix $ref paths (OpenAPI -> JSON Schema and external refs)
fn fix_refs(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(ref mut s)) = map.get_mut("$ref") {
                *s = s.replace("#/components/schemas/", "#/$defs/")
                       .replace("./schemas.yaml#/$defs/", "#/$defs/");
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