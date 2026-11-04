# phone-trie

Projet Rust ESGI : gestionnaire de numéros de téléphone avec un trie.

Le programme prend un fichier JSON de contacts, met les numéros dans un trie et génère
le fichier PlantUML (MindMap) pour voir l'arbre.

## Lancer

Il faut Rust (https://rustup.rs) puis :

```sh
cargo run --release -- data/04_common_parts.json
```

Le graphe est écrit dans `graph/`, avec le même nom que le json (ici
`graph/04_common_parts.puml`).

J'ai aussi ajouté la possibilité de chercher un numéro, on le met après le fichier :

```sh
cargo run --release -- data/04_common_parts.json 0412578440
```

Les tests : `cargo test`

## Infos

Le json doit avoir cette forme, les numéros sans +33 :

```json
[
    { "nb": "0412578440", "name": "Alice" },
    { "nb": "112", "name": "Urgences" }
]
```

Si un contact est pas bon (lettres dans le numéro, nom vide, numéro déjà pris) il est
juste ignoré avec un message, le reste est quand même traité. Si c'est le json entier
qui est cassé le programme s'arrête.

Le trie est fait à la main dans `src/trie.rs`, serde sert seulement à lire le json
(`src/contact.rs`) et la génération du PlantUML est dans `src/plantuml.rs`.

Pour les enfants d'un noeud j'ai pris un Vec plutôt qu'une HashMap pour garder l'ordre
d'insertion, sinon le graphe ne sort pas dans le même ordre que l'exemple du sujet. Il
y a 10 enfants max (0 à 9) donc c'est pas plus lent.

Dans `data/` il y a mes fichiers de test : liste vide, numéros sans début commun, un
numéro qui est le début d'un autre (112 et 1124), l'exemple du sujet et un fichier avec
des contacts invalides.

## Voir le graphe

```sh
docker run -d -p 8080:8080 plantuml/plantuml-server:jetty
```

puis aller sur http://localhost:8080/ et coller le contenu du .puml.
