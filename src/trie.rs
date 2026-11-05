//! Trie (arbre préfixe) de numéros de téléphone, implémenté sans crate externe.

use std::error::Error;
use std::fmt;

/// Raison pour laquelle un contact ne peut pas être inséré dans le trie.
#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    /// Le numéro est vide ou contient autre chose que des chiffres.
    InvalidNumber(String),
    /// Le nom est vide ou tient sur plusieurs lignes (PlantUML ne le lirait pas).
    InvalidName(String),
    /// Le numéro est déjà attribué à un contact.
    DuplicateNumber(String),
}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumber(number) => write!(f, "numéro invalide : {number:?}"),
            Self::InvalidName(name) => write!(f, "nom invalide : {name:?}"),
            Self::DuplicateNumber(number) => write!(f, "numéro déjà attribué : {number:?}"),
        }
    }
}

impl Error for InsertError {}

/// Noeud du trie : un noeud correspond à un préfixe de numéro.
///
/// Le chiffre est porté par le lien parent -> enfant, ce qui évite de donner
/// un chiffre factice à la racine. Les enfants sont dans un `Vec` pour garder
/// l'ordre d'insertion, attendu dans le graphe. Il y a au plus 10 enfants :
/// les parcourir reste donc très rapide.
#[derive(Debug, Default)]
pub struct Node {
    children: Vec<(char, Node)>,
    name: Option<String>,
}

impl Node {
    /// Enfants du noeud, avec leur chiffre, dans l'ordre d'insertion.
    pub fn children(&self) -> &[(char, Node)] {
        &self.children
    }

    /// Nom du contact dont le numéro se termine sur ce noeud, s'il y en a un.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn child(&self, digit: char) -> Option<&Node> {
        self.children
            .iter()
            .find(|(child_digit, _)| *child_digit == digit)
            .map(|(_, child)| child)
    }

    /// Renvoie l'enfant portant `digit`, en le créant s'il n'existe pas encore.
    fn child_or_insert(&mut self, digit: char) -> &mut Node {
        let existing = self
            .children
            .iter()
            .position(|(child_digit, _)| *child_digit == digit);

        let index = existing.unwrap_or_else(|| {
            self.children.push((digit, Node::default()));
            self.children.len() - 1
        });

        &mut self.children[index].1
    }
}

/// Annuaire stocké sous forme de trie.
#[derive(Debug, Default)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    /// Racine du trie : elle ne porte aucun chiffre ni aucun nom.
    pub fn root(&self) -> &Node {
        &self.root
    }

    /// Ajoute un contact. En cas d'erreur, le trie reste inchangé.
    pub fn insert(&mut self, number: &str, name: &str) -> Result<(), InsertError> {
        if number.is_empty() || !number.chars().all(|c| c.is_ascii_digit()) {
            return Err(InsertError::InvalidNumber(number.to_string()));
        }
        if name.trim().is_empty() || name.contains(['\n', '\r']) {
            return Err(InsertError::InvalidName(name.to_string()));
        }

        let mut node = &mut self.root;
        for digit in number.chars() {
            node = node.child_or_insert(digit);
        }

        if node.name.is_some() {
            return Err(InsertError::DuplicateNumber(number.to_string()));
        }
        node.name = Some(name.to_string());
        Ok(())
    }

    /// Cherche le nom associé à un numéro exact.
    pub fn get(&self, number: &str) -> Option<&str> {
        let mut node = &self.root;
        for digit in number.chars() {
            node = node.child(digit)?;
        }
        node.name()
    }

    /// Renvoie les numéros et noms dont le numéro commence par `prefix`.
    pub fn search_prefix(&self, prefix: &str) -> Vec<(String, String)> {
        let mut node = &self.root;
        for digit in prefix.chars() {
            let Some(child) = node.child(digit) else {
                return Vec::new();
            };
            node = child;
        }

        let mut results = Vec::new();
        let mut number = prefix.to_string();
        collect_contacts(node, &mut number, &mut results);
        results
    }
}

fn collect_contacts(node: &Node, number: &mut String, results: &mut Vec<(String, String)>) {
    if let Some(name) = node.name() {
        results.push((number.clone(), name.to_string()));
    }

    for (digit, child) in node.children() {
        number.push(*digit);
        collect_contacts(child, number, results);
        number.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_nodes(node: &Node) -> usize {
        1 + node
            .children()
            .iter()
            .map(|(_, child)| count_nodes(child))
            .sum::<usize>()
    }

    #[test]
    fn get_returns_inserted_name() {
        let mut trie = Trie::new();
        trie.insert("0412578440", "Alice").unwrap();

        assert_eq!(trie.get("0412578440"), Some("Alice"));
    }

    #[test]
    fn get_returns_none_for_unknown_number() {
        let mut trie = Trie::new();
        trie.insert("112", "Urgences").unwrap();

        assert_eq!(trie.get("113"), None);
        assert_eq!(trie.get("1123"), None);
    }

    #[test]
    fn search_prefix_returns_matching_numbers_in_insertion_order() {
        let mut trie = Trie::new();
        trie.insert("0412", "Court").unwrap();
        trie.insert("041257", "Alice").unwrap();
        trie.insert("041298", "Bob").unwrap();
        trie.insert("0512", "Autre").unwrap();

        assert_eq!(
            trie.search_prefix("0412"),
            [
                ("0412".to_string(), "Court".to_string()),
                ("041257".to_string(), "Alice".to_string()),
                ("041298".to_string(), "Bob".to_string()),
            ]
        );
    }

    #[test]
    fn search_prefix_returns_empty_for_unknown_prefix() {
        let mut trie = Trie::new();
        trie.insert("112", "Urgences").unwrap();

        assert!(trie.search_prefix("99").is_empty());
    }

    #[test]
    fn prefix_of_a_number_is_not_a_contact() {
        let mut trie = Trie::new();
        trie.insert("112", "Urgences").unwrap();

        assert_eq!(trie.get("11"), None);
    }

    #[test]
    fn number_can_be_prefix_of_another_number() {
        let mut trie = Trie::new();
        trie.insert("1123", "Long").unwrap();
        trie.insert("112", "Court").unwrap();

        assert_eq!(trie.get("112"), Some("Court"));
        assert_eq!(trie.get("1123"), Some("Long"));
    }

    #[test]
    fn common_prefix_is_stored_once() {
        let mut trie = Trie::new();
        trie.insert("0412", "Alice").unwrap();
        trie.insert("0413", "Bob").unwrap();

        // racine + 0, 4, 1 partagés + 2 et 3
        assert_eq!(count_nodes(trie.root()), 6);
    }

    #[test]
    fn children_keep_insertion_order() {
        let mut trie = Trie::new();
        trie.insert("5", "Cinq").unwrap();
        trie.insert("1", "Un").unwrap();

        let digits: Vec<char> = trie.root().children().iter().map(|(d, _)| *d).collect();
        assert_eq!(digits, ['5', '1']);
    }

    #[test]
    fn rejects_invalid_numbers_without_changing_the_trie() {
        let mut trie = Trie::new();

        for number in ["", "04a2", "+33", "04 12", "٠١"] {
            assert_eq!(
                trie.insert(number, "Alice"),
                Err(InsertError::InvalidNumber(number.to_string()))
            );
        }
        assert!(trie.root().children().is_empty());
    }

    #[test]
    fn rejects_invalid_names_without_changing_the_trie() {
        let mut trie = Trie::new();

        for name in ["", "   ", "Alice\nBob"] {
            assert_eq!(
                trie.insert("112", name),
                Err(InsertError::InvalidName(name.to_string()))
            );
        }
        assert!(trie.root().children().is_empty());
    }

    #[test]
    fn duplicate_number_keeps_first_contact() {
        let mut trie = Trie::new();
        trie.insert("112", "Urgences").unwrap();

        assert_eq!(
            trie.insert("112", "Autre"),
            Err(InsertError::DuplicateNumber("112".to_string()))
        );
        assert_eq!(trie.get("112"), Some("Urgences"));
    }
}
