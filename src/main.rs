use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use phone_trie::contact::{parse_contacts, Contact};
use phone_trie::plantuml::to_mindmap;
use phone_trie::trie::Trie;

const GRAPH_DIR: &str = "graph";
const USAGE: &str = "usage : phone-trie <fichier.json> [numéro | --prefix préfixe]";

enum SearchMode<'a> {
    Exact(&'a str),
    Prefix(&'a str),
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let (input, search) = match args.as_slice() {
        [input] => (input, None),
        [input, number] => (input, Some(SearchMode::Exact(number))),
        [input, flag, prefix] if flag == "--prefix" => {
            (input, Some(SearchMode::Prefix(prefix)))
        }
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    match run(Path::new(input), search) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("erreur : {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(input: &Path, search: Option<SearchMode<'_>>) -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string(input).map_err(|e| format!("{}: {e}", input.display()))?;
    let contacts = parse_contacts(&json).map_err(|e| format!("{}: {e}", input.display()))?;

    let trie = build_trie(&contacts);

    let output = output_path(input)?;
    fs::create_dir_all(GRAPH_DIR)?;
    fs::write(&output, to_mindmap(&trie))?;
    println!("graphe généré : {}", output.display());

    match search {
        Some(SearchMode::Exact(number)) => match trie.get(number) {
            Some(name) => println!("{number} : {name}"),
            None => println!("{number} : inconnu"),
        },
        Some(SearchMode::Prefix(prefix)) => {
            let results = trie.search_prefix(prefix);
            if results.is_empty() {
                println!("aucun numéro ne commence par {prefix}");
            } else {
                for (number, name) in results {
                    println!("{number} : {name}");
                }
            }
        }
        None => {}
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
