use crate::parser::Model;
use tera::{Tera, Context, Value, to_value, Result as TeraResult};
use std::path::Path;
use anyhow::{Result, Context as AnyhowContext};
use std::fs;
use include_dir::{include_dir, Dir};
use std::collections::HashMap;

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/templates/javascript");

pub fn filter_js_type(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().unwrap_or("any");
    let type_lower = s.to_lowercase();
    
    let js_type = match type_lower.as_str() {
        "int" | "integer" | "float" | "real" | "double" | "decimal" | "long" | "number" => "number",
        "bool" | "boolean" => "boolean",
        "str" | "string" | "text" | "char" | "uuid" | "unique_id" | "email" => "string",
        "datetime" | "timestamp" | "date" => "Date",
        "json" | "map" | "binary" | "obj" | "inst_ref" | "jsonb" => "Object",
        "list" | "array" | "inst_ref_set" => "Array",
        "void" => "void",
        _ => "any",
    };

    Ok(to_value(js_type).unwrap())
}

fn load_tera() -> Result<Tera> {
    let mut tera = Tera::default();
    tera.register_filter("js_type", filter_js_type);
    fn register_dir(tera: &mut Tera, dir: &Dir) -> Result<()> {
        for file in dir.files() {
            if let Some(content) = file.contents_utf8() {
                let path = file.path();
                if let Some(filename) = path.file_name() {
                     let name_str = filename.to_string_lossy().to_string();
                     if name_str.ends_with(".tera") {
                        tera.add_raw_template(&name_str, content)?;
                     }
                }
            }
        }
        for subdir in dir.dirs() {
            register_dir(tera, subdir)?;
        }
        Ok(())
    }
    
    print!("Loading templates...\n");
    register_dir(&mut tera, &TEMPLATE_DIR)?;

    Ok(tera)
}

fn safe_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

pub fn generate(model: &Model, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)?;
    let tera = load_tera()?;
    let mut ctx_header = Context::new();
    ctx_header.insert("model_name", &model.model_name);
    ctx_header.insert("version", &model.version);

    let header = tera.render("header.js.tera", &ctx_header)?;
    let footer = tera.render("footer.js.tera", &Context::new())?;

    for cls in &model.classes {
        println!("Generate class: {:?}", &cls.name);
        let mut combined = String::new();
        combined.push_str(&header);
        combined.push('\n');

        let mut ctx = Context::new();
        ctx.insert("class", &cls);

        let class_events: Vec<_> = model.events
            .iter()
            .filter(|ev| ev.trigger.as_deref() == Some(&cls.name))
            .collect();
        ctx.insert("associations", &model.associations);

        ctx.insert("class_events", &class_events);

        combined.push_str(&tera.render("class.js.tera", &ctx)?);
        combined.push('\n');
        combined.push_str(&footer);

        let filename = format!("{}.js", safe_filename(&cls.name));
        let out_path = out_dir.join(filename);

        fs::write(&out_path, combined)
            .with_context(|| format!("Failed to write class file {:?}", out_path))?;
    }

    // Runtime
    let runtime_code = tera.render("runtime.js.tera", &Context::new())?;
    let runtime_path = out_dir.join("Runtime.js");
    fs::write(&runtime_path, runtime_code)
        .with_context(|| format!("Failed to write runtime file {:?}", runtime_path))?;
    
    // Event 
    let mut combined = String::new();
    combined.push_str(&header);
    combined.push('\n');

    let mut ctx_events = Context::new();
    ctx_events.insert("events", &model.events);
    combined.push_str(&tera.render("event.js.tera", &ctx_events)?);
    combined.push('\n');
    combined.push_str(&footer);

    let events_path = out_dir.join("events.js");
    fs::write(&events_path, combined)
        .with_context(|| format!("Failed to write events file {:?}", events_path))?;

    Ok(())
}
