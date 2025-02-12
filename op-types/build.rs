use std::env;
use std::path::PathBuf;
use std::{fs, process::Command};
use openapi_to_jsonschema;
use schemars::schema::RootSchema;

fn main() {
    let definitions_path = env::var("OPENAPI_DEFINITIONS_PATH")
        .unwrap_or_else(|_| "../openapi-definitions".to_string());

    let auth_server_spec = format!("{}/auth-server.yaml", definitions_path);
    let resource_server_spec =   format!("{}/resource-server.yaml", definitions_path);
    let wallet_address_server_spec =    format!("{}/wallet-address-server.yaml", definitions_path);

    let conversions = [
        (auth_server_spec.as_str(), "./generated/auth-server-schema.json"),
        (resource_server_spec.as_str(), "./generated/resource-server-schema.json"),
        (wallet_address_server_spec.as_str(), "./generated/wallet-address-server-schema.json"),
    ];

    let shared_schemas_path: String = format!("{}/schemas.yaml", definitions_path);

    
    openapi_to_jsonschema::convert_openapi_to_jsonschema_with_shared_schema(&conversions, &shared_schemas_path)
        .unwrap_or_else(|e| panic!("Conversion failed: {}", e));
    

    let schemas = [
        "./generated/auth-server-schema.json",
        "./generated/resource-server-schema.json",
        "./generated/wallet-address-server-schema.json",
    ];

    let mut modules = String::from("// Auto-generated modules\n");

    for schema_path in &schemas {
        let mut type_space = typify::TypeSpace::default();

        let schema_content = fs::read_to_string(schema_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", schema_path, e));

        let mut schema: RootSchema = serde_json::from_str(&schema_content)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {}", schema_path, e));

        // This solves wallet address types conflict
        rename_definition(&mut schema, "walletAddress", "walletAddressURI");

        type_space.add_root_schema(schema)
            .unwrap_or_else(|e| panic!("Failed to add schema {}: {}", schema_path, e));

        // Generate output filename
        let file_name = PathBuf::from(schema_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .replace("-server-schema", "")
            .replace("-", "_");
        
        let out_path = PathBuf::from("src").join(format!("{}.rs", file_name));

        // Write generated types
        let contents = type_space.to_stream().to_string();
        fs::write(&out_path, &contents)
            .unwrap_or_else(|e| panic!("Failed to write {}: {}", out_path.display(), e));

        Command::new("rustfmt")
            .arg(&out_path)
            .status()
            .expect("Failed to run rustfmt");

        modules.push_str(&format!("pub mod {};\n", file_name));
    }

    let lib_path = PathBuf::from("src/lib.rs");
    fs::write(&lib_path, modules)
        .unwrap_or_else(|e| panic!("Failed to update lib.rs: {}", e));

    Command::new("rustfmt")
        .arg(&lib_path)
        .status()
        .expect("Failed to format lib.rs");
}

fn rename_definition(root_schema: &mut RootSchema, old_key: &str, new_key: &str) {
    if let Some(schema) = root_schema.definitions.remove(old_key) {
        root_schema.definitions.insert(new_key.to_string(), schema);
    }
}