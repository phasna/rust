//! Lecture des contacts depuis le JSON d'entrée.

use serde::Deserialize;

/// Contact tel que décrit dans le fichier JSON.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Contact {
    #[serde(rename = "nb")]
    pub number: String,
    pub name: String,
}

/// Transforme le contenu d'un fichier JSON en liste de contacts.
pub fn parse_contacts(json: &str) -> Result<Vec<Contact>, serde_json::Error> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_contacts() {
        let contacts = parse_contacts(r#"[{ "nb": "112", "name": "Urgences" }]"#).unwrap();

        assert_eq!(
            contacts,
            [Contact {
                number: "112".to_string(),
                name: "Urgences".to_string(),
            }]
        );
    }

    #[test]
    fn parses_empty_list() {
        assert!(parse_contacts("[]").unwrap().is_empty());
    }

    #[test]
    fn rejects_malformed_json() {
        assert!(parse_contacts(r#"[{ "nb": "112" "#).is_err());
    }

    #[test]
    fn rejects_contact_without_name() {
        assert!(parse_contacts(r#"[{ "nb": "112" }]"#).is_err());
    }
}
