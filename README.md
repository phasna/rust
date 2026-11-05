# phone-trie

Gestionnaire de numéros de téléphone en Rust. Le programme lit une liste de contacts dans un fichier JSON, les range dans un trie (arbre préfixe) et écrit le fichier PlantUML MindMap qui représente cet arbre.

Le trie est écrit à la main dans `src/trie.rs`. La seule dépendance externe est `serde`, utilisée uniquement pour lire le JSON. `#![forbid(unsafe_code)]` est placé dans `src/lib.rs` : le projet ne compile pas s'il contient du code `unsafe`.

PlantUML n'est pas lancé par le programme. Le `.puml` est produit dans `graph/`, puis visualisé à la main.

## Prérequis

- Rust, via [rustup](https://rustup.rs)
- Docker, seulement pour afficher le graphe dans le navigateur

Depuis la racine du dépôt :

```sh
cargo run --release -- data/04_common_parts.json
```

## Lancer

Un argument : le fichier JSON. Le programme construit le trie et écrit le MindMap.

```sh
cargo run --release -- data/04_common_parts.json
```

Sortie :

```text
graphe généré : graph/04_common_parts.puml
```

Le fichier de sortie reprend le nom du JSON. `data/exemple.json` donne `graph/exemple.puml`. Le dossier `graph/` est créé s'il n'existe pas.

Un second argument cherche ce numéro dans le trie, après la génération du graphe :

```sh
cargo run --release -- data/04_common_parts.json 0412578440
```

```text
graphe généré : graph/04_common_parts.puml
0412578440 : Alice
```

Un numéro absent affiche `inconnu` :

```sh
cargo run --release -- data/04_common_parts.json 999
```

```text
999 : inconnu
```

Sans argument, ou avec plus de deux arguments, le programme affiche l'usage et s'arrête :

```text
usage : phone-trie <fichier.json> [numéro à rechercher]
```

Les tests :

```sh
cargo test
```

## Format du JSON

Le fichier est une liste d'objets. Chaque objet a un numéro `nb` et un nom `name`. Les deux sont des chaînes. Les numéros n'ont pas de préfixe international (`+33`) : ce sont uniquement des chiffres, et le `0` initial est conservé parce que le numéro n'est pas converti en entier.

```json
[
    { "nb": "0412578440", "name": "Alice" },
    { "nb": "0412199803", "name": "Bob" },
    { "nb": "0468892011", "name": "patate" },
    { "nb": "112", "name": "Urgences" },
    { "nb": "15", "name": "SAMU" }
]
```

C'est le contenu de `data/04_common_parts.json`, l'exemple du sujet.

## Contacts invalides

Un contact inutilisable est signalé sur la sortie d'erreur, puis ignoré. Les contacts suivants sont quand même insérés.

| Cas | Exemple | Résultat |
|---|---|---|
| Lettre ou caractère autre qu'un chiffre | `04a2`, `+33`, `04 12` | ignoré |
| Numéro vide | `""` | ignoré |
| Nom vide ou composé seulement d'espaces | `""`, `"   "` | ignoré |
| Nom sur plusieurs lignes | une chaîne contenant un retour à la ligne | ignoré |
| Numéro déjà présent | le même `nb` une seconde fois | ignoré, le premier nom est conservé |

```sh
cargo run --release -- data/05_invalid_entries.json
```

Le fichier contient Alice, un doublon d'Alice, un numéro avec une lettre, un numéro vide, un nom vide, puis le SAMU. Le terminal affiche un `contact ignoré` pour chaque entrée refusée. Le graphe `graph/05_invalid_entries.puml` ne contient qu'Alice et le SAMU.

Le fichier entier doit rester du JSON valide, avec `nb` et `name` sur chaque objet. Si le fichier est introuvable, si le JSON est tronqué, ou s'il manque un champ, la lecture échoue et le programme s'arrête avec `erreur : …` et un code de sortie d'échec. Aucun graphe n'est écrit dans ce cas.

## Le trie

Chaque nœud correspond à un préfixe de numéro. Le chiffre est porté par le lien vers l'enfant, pas par le nœud : la racine n'a donc ni chiffre ni nom. Le nom n'est enregistré que sur le nœud où le numéro se termine. Un nœud intermédiaire n'est pas un contact.

Alice (`0412578440`) et Bob (`0412199803`) partagent le chemin `0 → 4 → 1 → 2`. Ce préfixe n'existe qu'une fois en mémoire. Le `5` mène ensuite à Alice, le `1` à Bob. `0468892011` quitte ce tronc au `6`. `112` et `15` ne partagent que le premier `1`.

```text
        0 → 4 → 1 → 2 → 5 → 7 → 8 → 4 → 4 → 0 → Alice
                      └→ 1 → 9 → 9 → 8 → 0 → 3 → Bob
                └→ 6 → 8 → 8 → 9 → 2 → 0 → 1 → 1 → patate
        1 → 1 → 2 → Urgences
          └→ 5 → SAMU
```

Insérer ou chercher un numéro descend d'un nœud par chiffre. Le nombre de pas est la longueur du numéro, indépendamment du nombre de contacts. Pour `0412578440`, ce sont dix pas.

Un numéro peut être le préfixe d'un autre. Dans `data/03_number_is_prefix.json`, `112` (Urgences), `1124` (Standard) et `11245` (Accueil) coexistent. Le nœud du dernier `2` porte le nom `Urgences` et possède aussi un enfant `4`. Chercher `11` ne renvoie personne : le nom est sur le dernier chiffre, pas sur les nœuds du milieu.

Les enfants d'un nœud sont dans un `Vec<(chiffre, nœud)>`, dans l'ordre d'insertion. Une `HashMap` ne garantit pas cet ordre, et le MindMap du sujet suit l'ordre du JSON : le `0` avant le `1`, et chez Alice le `5` avant le `1` de Bob. Un nœud a au plus dix enfants, les chiffres de `0` à `9`.

Le sujet demande l'insertion et le parcours. Le programme expose `insert` et `get`. Il n'y a pas de suppression ni de remplacement d'un contact déjà inséré.

## Fichier PlantUML

`src/plantuml.rs` parcourt le trie et écrit un MindMap. Le nombre d'étoiles est la profondeur. S'il y a un nom sur le nœud, il est écrit avant les chiffres qui continuent après lui. C'est ce qui rend visible un numéro préfixe d'un autre.

Pour l'exemple du sujet, le début et la fin du fichier sont :

```text
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
...
* 1
** 1
*** 2
**** Urgences
** 5
*** SAMU
@endmindmap
```

Le fichier complet généré est `graph/04_common_parts.puml`. Le test `matches_the_example_of_the_subject` compare cette forme au texte attendu.

Pour `112` puis `1124`, la sortie contient :

```text
* 1
** 1
*** 2
**** Urgences
**** 4
***** Standard
```

Une liste vide produit un MindMap sans branche :

```text
@startmindmap
@endmindmap
```

## Organisation

```text
src/lib.rs        modules et #![forbid(unsafe_code)]
src/main.rs       arguments, lecture du fichier, écriture du .puml, recherche
src/contact.rs    struct Contact et parse_contacts (serde)
src/trie.rs       Node, Trie, insert, get
src/plantuml.rs   to_mindmap
data/             JSON de test
graph/            MindMap générés
```

`main` enchaîne quatre étapes : lire le fichier, désérialiser la liste, insérer chaque contact, écrire le `.puml`. La recherche est faite ensuite, seulement si un numéro a été passé en argument.

## Jeux de test

| Fichier | Ce qu'il vérifie |
|---|---|
| `data/01_empty.json` | liste vide, MindMap sans branche |
| `data/02_no_common_parts.json` | `0123`, `4567` et `89` ne partagent aucun préfixe : trois branches séparées |
| `data/03_number_is_prefix.json` | `112`, `1124` et `11245` : un numéro est le début d'un autre |
| `data/04_common_parts.json` | l'exemple du sujet (Alice, Bob, patate, Urgences, SAMU) |
| `data/05_invalid_entries.json` | doublon, lettre, numéro vide, nom vide, puis un contact valide |

Les graphes déjà générés pour ces fichiers sont dans `graph/`, avec le même nom et l'extension `.puml`.

`cargo test` couvre en plus la recherche d'un numéro inséré, un numéro inconnu, un préfixe qui n'est pas un contact, le partage d'un préfixe (Alice `0412` et Bob `0413` donnent 6 nœuds : la racine, `0`, `4`, `1`, puis `2` et `3`), l'ordre d'insertion des enfants, et le fait qu'une insertion refusée ne modifie pas le trie.

## Voir le graphe

Démarrer le serveur PlantUML :

```sh
docker run -d -p 8080:8080 plantuml/plantuml-server:jetty
```

Ouvrir http://localhost:8080/ et coller le contenu d'un fichier de `graph/`. L'image obtenue pour `04_common_parts.puml` est le trie de l'exemple : le tronc `0412` commun à Alice et Bob, la branche de patate, puis Urgences et SAMU sous le `1`.
