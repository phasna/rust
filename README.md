# Gestionnaire de numéros de téléphone

Lit des contacts dans un fichier JSON, les stocke dans un **trie** (arbre préfixe)
implémenté from scratch, et génère le diagramme PlantUML MindMap de la structure.

Dans un trie, les numéros qui commencent pareil partagent les mêmes noeuds : le
préfixe n'est stocké qu'une fois, et une recherche ne dépend que de la longueur du
numéro, pas du nombre de contacts.

## Prérequis

[Rust](https://rustup.rs) (cargo).

## Lancer le projet

```sh
# Génère graph/04_common_parts.puml
cargo run --release -- data/04_common_parts.json

# Génère le graphe, puis recherche un numéro dans le trie
cargo run --release -- data/04_common_parts.json 0412578440
```

Le fichier généré porte le nom du fichier d'entrée, dans le dossier `graph/`.

## Lancer les tests

```sh
cargo test
```

## Format d'entrée

```json
[
    { "nb": "0412578440", "name": "Alice" },
    { "nb": "112", "name": "Urgences" }
]
```

Les numéros n'ont pas de préfixe international et ne contiennent que des chiffres.

Une erreur de lecture ou un JSON mal formé arrête le programme. Un contact invalide
(numéro non numérique, nom vide, numéro déjà attribué) est signalé sur la sortie
d'erreur puis ignoré : le graphe est quand même généré, et le premier contact d'un
numéro est conservé.

## Visualiser un graphe

```sh
docker pull plantuml/plantuml-server:jetty
docker run -d -p 8080:8080 plantuml/plantuml-server:jetty
```

Ouvrir <http://localhost:8080/> puis y coller le contenu d'un fichier `.puml`.

## Organisation

| Chemin            | Rôle                                         |
| ----------------- | -------------------------------------------- |
| `src/trie.rs`     | Trie : insertion et recherche                |
| `src/contact.rs`  | Lecture du JSON avec `serde`                 |
| `src/plantuml.rs` | Parcours du trie et génération du MindMap    |
| `src/main.rs`     | Ligne de commande                            |
| `data/`           | Jeux de données de test                      |
| `graph/`          | Fichiers PlantUML générés                    |

Les enfants d'un noeud sont gardés dans un `Vec`, et non une `HashMap`, pour conserver
l'ordre d'insertion attendu dans le graphe. Un noeud ayant au plus 10 enfants, le
parcours de ce `Vec` reste très rapide.
