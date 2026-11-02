use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use phone_trie::contact::{parse_contacts, Contact};
use phone_trie::plantuml::to_mindmap;
use phone_trie::trie::Trie;

const GRAPH_DIR: &str = "graph";
const USAGE: &str = "usage : phone-trie <fichier.json> [numéro à rechercher]";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let (input, searched_number) = match args.as_slice() {
        [input] => (input, None),
        [input, number] => (input, Some(number)),
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    match run(Path::new(input), searched_number.map(String::as_str)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("erreur : {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(input: &Path, searched_number: Option<&str>) -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string(input).map_err(|e| format!("{}: {e}", input.display()))?;
    let contacts = parse_contacts(&json).map_err(|e| format!("{}: {e}", input.display()))?;

    let trie = build_trie(&contacts);

    let output = output_path(input)?;
    fs::create_dir_all(GRAPH_DIR)?;
    fs::write(&output, to_mindmap(&trie))?;
    println!("graphe généré : {}", output.display());

    if let Some(number) = searched_number {
        match trie.get(number) {
            Some(name) => println!("{number} : {name}"),
            None => println!("{number} : inconnu"),
        }
    }

    Ok(())
}

/// Construit le trie ; un contact invalide est signalé puis ignoré.
fn build_trie(contacts: &[Contact]) -> Trie {
    let mut trie = Trie::new();
    for contact in contacts {
        if let Err(error) = trie.insert(&contact.number, &contact.name) {
            eprintln!("contact ignoré ({}) : {error}", contact.name);
        }
    }
    trie
}

/// `data/exemple.json` donne `graph/exemple.puml`.
fn output_path(input: &Path) -> Result<PathBuf, String> {
    let stem = input
        .file_stem()
        .ok_or_else(|| format!("nom de fichier invalide : {}", input.display()))?;
    Ok(Path::new(GRAPH_DIR).join(format!("{}.puml", stem.to_string_lossy())))
}
