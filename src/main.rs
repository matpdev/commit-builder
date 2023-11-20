use dialoguer::Select;
use std::process::{ Command, Output };
use dialoguer::Input;

fn main() {
    let items = vec!["HOTFIX", "FIX", "UPDATE", "RELEASE", "CI", "CUSTOM"];
    let mut commit = String::new();

    let selection = Select::new()
        .with_prompt("Choose a commit type")
        .items(&items)
        .interact()
        .unwrap();

    if selection != 5 {
        commit.push_str(items[selection]);
    }

    if selection == 3 {
        let version: String = Input::new().with_prompt("Digite a versão").interact_text().unwrap();
        commit.push_str(" ");
        commit.push_str(version.to_owned().as_str());
    }

    if selection == 5 {
        let type_data: String = Input::new()
            .with_prompt("Digite o tipo que deseja")
            .interact_text()
            .unwrap();
        commit.push_str(&type_data.as_str());
        let commit_data: String = Input::new()
            .with_prompt("Agora o comentário do commit")
            .interact_text()
            .unwrap();
        commit.push_str(" ");
        commit.push_str(&commit_data.to_owned().as_str());
    } else {
        let commit_data: String = Input::new()
            .with_prompt("Agora o comentário do commit")
            .interact_text()
            .unwrap();
        commit.push_str(": ");
        commit.push_str(&commit_data.as_str());
    }

    run_command(&commit);
}

fn run_command(str_co: &str) {
    Command::new("git").args(["add", "."]).output().expect("Ocorreu um erro ao commitar");

    Command::new("git")
        .args(["commit", "-m", str_co])
        .output()
        .expect("Ocorreu um erro ao commitar");
}
