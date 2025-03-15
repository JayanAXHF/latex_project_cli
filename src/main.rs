use inquire::Select;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env::{self, current_dir},
    fs::{self, create_dir_all},
    io::{self, Write},
    path::Path,
};

#[derive(Serialize, Deserialize)]
struct Template {
    name: String,
    path: String,
}

#[derive(Serialize, Deserialize)]
struct Json {
    templates: Vec<Template>,
}

fn main() -> io::Result<()> {
    let mut project_name = String::new();
    let mut args = env::args();
    args.next();
    if let Some(t) = args.next() {
        project_name = t;
    } else {
        print!("Enter The project name: ");
        let _ = io::stdout().flush();
        let result = io::stdin().read_line(&mut project_name);
        project_name = match result {
            Ok(_) => project_name.trim().to_string(),
            _ => String::from("template"),
        };
    }

    let parsed_json: Json = serde_json::from_str(include_str!("../templates.json")).unwrap();
    let available_templates = parsed_json
        .templates
        .iter()
        .map(|t| &t.name)
        .collect::<Vec<_>>();

    let template = Select::new("Select Template", available_templates.to_vec())
        .prompt()
        .unwrap();

    let templates = parsed_json
        .templates
        .iter()
        .map(|t| (t.name.clone(), get_path(&t.path)))
        .collect::<Vec<_>>();

    let mapping = templates.into_iter().collect::<HashMap<_, _>>();

    let current_dir = current_dir()?;
    let project_dir = current_dir.join(&project_name);
    create_dir_all(&project_dir)?;
    let _ = fs::create_dir(current_dir.join(&project_name));
    for entry in fs::read_dir(Path::new(mapping.get(template).unwrap())).unwrap() {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let _ = fs::write(current_dir.join(&project_name), "");
        let _ = fs::copy(path, project_dir.join(name));
    }
    Ok(())
}

/// Adds the home directory to the path
fn get_path(partial_path: &str) -> String {
    let mut partial_path = partial_path.to_string().chars().collect::<Vec<_>>();
    partial_path.remove(0);
    let partial_path = partial_path.into_iter().collect::<String>();
    let home_dir = dirs::home_dir().unwrap();
    let home_dir = home_dir.as_path();
    let complete_dir = home_dir.join(partial_path);
    complete_dir.to_str().unwrap().to_owned()
}
