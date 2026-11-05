# phone-trie

Projet Rust de l'ESGI : un gestionnaire de numéros de téléphone.

Le programme lit une liste de contacts dans un fichier JSON, les range dans un trie
(un arbre préfixe) et génère un fichier PlantUML pour voir à quoi ressemble l'arbre.
PlantUML n'est pas appelé par le programme, je génère juste le `.puml`.

## Lancer le projet

Il faut avoir Rust installé (https://rustup.rs).

```sh
cargo run --release -- data/04_common_parts.json
```

Ça crée `graph/04_common_parts.puml`. Le fichier de sortie prend le même nom que le
JSON donné en entrée, et le dossier `graph/` est créé s'il n'existe pas.

On peut aussi passer un numéro en deuxième argument pour le chercher dans le trie :

```sh
cargo run --release -- data/04_common_parts.json 0412578440
# 0412578440 : Alice
```

Si le numéro n'est pas dans le trie, le programme affiche `inconnu`.

On peut aussi rechercher tous les contacts dont le numéro commence par un préfixe :

```sh
cargo run --release -- data/04_common_parts.json --prefix 0412
# 0412578440 : Alice
# 0412199803 : Bob
```

Et pour les tests :

```sh
cargo test
```

## Le fichier JSON

```json
[
    { "nb": "0412578440", "name": "Alice" },
    { "nb": "112", "name": "Urgences" }
]
```

Pas de `+33` devant les numéros, juste des chiffres. Je les garde en texte, sinon
le `0` du début saute si on en fait un nombre.

Si le fichier n'existe pas ou que le JSON est cassé, le programme s'arrête avec une
erreur. Par contre, si c'est juste un contact qui pose problème (lettres dans le
numéro, nom vide, numéro en double), je l'affiche dans le terminal et je passe au
suivant. Je trouvais dommage de tout bloquer pour une seule ligne fausse. Pour les
doublons, c'est le premier contact qui reste.

`data/05_invalid_entries.json` sert à vérifier ça : Alice et le SAMU restent, le
reste est ignoré.

## Comment ça marche

L'idée du trie : chaque chiffre est un pas dans l'arbre, et les numéros qui
commencent pareil partagent le même chemin. Par exemple `0412578440` (Alice) et
`0412199803` (Bob) passent tous les deux par `0 → 4 → 1 → 2`, et ce début n'est
stocké qu'une fois. Le nom est mis seulement sur le dernier chiffre. Du coup `112`
peut exister en même temps que `1124` : Urgences est sur le `2`, et le `4` continue
après. Pour retrouver un numéro, on descend chiffre par chiffre, donc ça dépend de
la longueur du numéro et pas du nombre de contacts.

Le code est découpé comme ça :

- `src/trie.rs` : le trie (insertion et recherche), écrit sans crate
- `src/contact.rs` : la lecture du JSON avec `serde`
- `src/plantuml.rs` : le parcours de l'arbre pour écrire le MindMap
- `src/main.rs` : la ligne de commande
- `data/` : mes fichiers de test
- `graph/` : les `.puml` générés

`#![forbid(unsafe_code)]` est dans `src/lib.rs`.

Un choix que j'ai fait : les enfants d'un noeud sont dans un `Vec` et pas dans une
`HashMap`. Avec une `HashMap`, l'ordre n'est pas garanti, et je voulais que le graphe
sorte dans le même ordre que l'exemple du sujet. Comme un noeud a au maximum
10 enfants (les chiffres de 0 à 9), parcourir le `Vec` ne coûte quasiment rien.

Dans le `.puml`, le nombre d'étoiles correspond à la profondeur. Le fichier généré
pour l'exemple du sujet est `graph/04_common_parts.puml`.

## Les jeux de test

- `01_empty.json` : liste vide
- `02_no_common_parts.json` : aucun numéro ne partage de début
- `03_number_is_prefix.json` : un numéro qui est le début d'un autre (`112` et `1124`)
- `04_common_parts.json` : l'exemple du sujet
- `05_invalid_entries.json` : des contacts faux, pour vérifier qu'ils sont bien ignorés

## Voir le graphe

Avec Docker :

```sh
docker run -d -p 8080:8080 plantuml/plantuml-server:jetty
```

Ensuite aller sur http://localhost:8080/ et coller le contenu d'un `.puml`.
