//! Génération du diagramme PlantUML MindMap d'un trie.

use crate::trie::{Node, Trie};

/// Génère le code PlantUML MindMap représentant le trie.
pub fn to_mindmap(trie: &Trie) -> String {
    let mut output = String::from("@startmindmap\n");
    write_children(&mut output, trie.root(), 1);
    output.push_str("@endmindmap\n");
    output
}

/// Écrit le contenu de `node`, `depth` étant le nombre d'étoiles de ses enfants.
fn write_children(output: &mut String, node: &Node, depth: usize) {
    if let Some(name) = node.name() {
        write_line(output, depth, name);
    }
    for (digit, child) in node.children() {
        write_line(output, depth, &digit.to_string());
        write_children(output, child, depth + 1);
    }
}

fn write_line(output: &mut String, depth: usize, label: &str) {
    output.push_str(&"*".repeat(depth));
    output.push(' ');
    output.push_str(label);
    output.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_trie_gives_empty_mindmap() {
        assert_eq!(to_mindmap(&Trie::new()), "@startmindmap\n@endmindmap\n");
    }

    #[test]
    fn number_and_prefix_are_both_displayed() {
        let mut trie = Trie::new();
        trie.insert("12", "Court").unwrap();
        trie.insert("123", "Long").unwrap();

        assert_eq!(
            to_mindmap(&trie),
            "@startmindmap\n* 1\n** 2\n*** Court\n*** 3\n**** Long\n@endmindmap\n"
        );
    }

    #[test]
    fn matches_the_example_of_the_subject() {
        let mut trie = Trie::new();
        trie.insert("0412578440", "Alice").unwrap();
        trie.insert("0412199803", "Bob").unwrap();
        trie.insert("0468892011", "Patate").unwrap();
        trie.insert("112", "Urgences").unwrap();
        trie.insert("15", "SAMU").unwrap();

        let expected = "\
@startmindmap
* 0
** 4
*** 1
**** 2
***** 5
****** 7
******* 8
******** 4
********* 4
********** 0
*********** Alice
***** 1
****** 9
******* 9
******** 8
********* 0
********** 3
*********** Bob
*** 6
**** 8
***** 8
****** 9
******* 2
******** 0
********* 1
********** 1
*********** Patate
* 1
** 1
*** 2
**** Urgences
** 5
*** SAMU
@endmindmap
";
        assert_eq!(to_mindmap(&trie), expected);
    }
}
