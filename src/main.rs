use std::path::PathBuf;
use std::{
    collections::HashMap,
    env::{self, current_dir},
    fs::{self, create_dir_all},
    io::{self, Write},
    path::Path,
};

use inquire::Select;

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

    let available_templates = [
        "PNAS-style numbered-lines(single column)",
        "PNAS-Style no line number (double column)",
        "Styled Report (one-column)",
        "Global Leaders Delegate Research",
        "Position Paper Template",
        "Draft Resolution Template",
        "Multipurpose Report Template",
    ];
    let template = Select::new("Select Template", available_templates.to_vec())
        .prompt()
        .unwrap();
    let mapping = HashMap::from([
        (
            available_templates[0],
            get_path("/.config/templates/latex/template_num_lines"),
        ),
        (
            available_templates[1],
            get_path("/.config/templates/latex/template_double_column"),
        ),
        (
            available_templates[2],
            get_path("/.config/templates/latex/template_report_styled"),
        ),
        (
            available_templates[3],
            get_path("/.config/templates/latex/template_research_global_leaders"),
        ),
        (
            available_templates[4],
            get_path("/.config/templates/latex/template_position_paper"),
        ),
        (
            available_templates[5],
            get_path("/.config/templates/latex/template_draft_resolution"),
        ),
        (
            available_templates[6],
            get_path("/.config/templates/latex/template_multipurpose"),
        ),
    ]);

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

fn get_path(partial_path: &str) -> String {
    let mut partial_path = partial_path.to_string().chars().collect::<Vec<_>>();
    partial_path.remove(0);
    let partial_path = partial_path.into_iter().collect::<String>();
    let home_dir = dirs::home_dir().unwrap();
    let home_dir = home_dir.as_path();
    let complete_dir = home_dir.join(partial_path);
    complete_dir.to_str().unwrap().to_owned()
}
