use std::{env::current_dir, fs::create_dir_all};

use cosmwasm_schema::{export_schema, remove_schemas};
use schemars::schema_for;
use serde_json::json;

use instantiator::{
    msg::{ExecuteMsg, InitMsg},
    state::State,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    // Generate schema for InitMsg
    let schema = schema_for!(InitMsg);
    let schema_json = json!(&schema).to_string();
    println!("InitMsg schema:\n{}", schema_json);
    export_schema(&schema_for!(InitMsg), &out_dir);

    // Generate schema for State
    let schema = schema_for!(State);
    let schema_json = json!(&schema).to_string();
    println!("\nState schema:\n{}", schema_json);
    export_schema(&schema_for!(State), &out_dir);

    // Generate schema for Execute
    let schema = schema_for!(ExecuteMsg);
    let schema_json = json!(&schema).to_string();
    println!("\nExecuteMsg schema:\n{}", schema_json);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
}
