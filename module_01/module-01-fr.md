---

## Exercice 00 : Création de Références

```txt
Répertoire de rendu :
    ex00/

Fichiers à rendre :
    src/lib.rs  Cargo.toml
```

Créez deux **fonctions**. Les deux doivent additionner deux entiers.

```rust
fn add(a: &i32, b: i32) -> i32;
fn add_assign(a: &mut i32, b: i32);
```

* `add` doit retourner le résultat de l'opération.
* `add_assign` doit stocker le résultat de l'opération dans `a`.

---

## Exercice 01 : Le Point de Non-Retour (v2)

```txt
Répertoire de rendu :
    ex01/

Fichiers à rendre :
    src/lib.rs  Cargo.toml
```

Écrivez une **fonction** qui retourne la plus petite valeur parmi deux nombres.

```rust
fn min(a: &i32, b: &i32) -> &i32;
```

* Notez que vous devrez peut-être ajouter des *annotations de durée de vie* à la fonction pour qu'elle compile.
* Le mot-clé `return` est toujours interdit.

---

## Exercice 02 : Le Nom des Couleurs

```txt
Répertoire de rendu :
    ex02/

Fichiers à rendre :
    src/lib.rs  Cargo.toml
```

Créez une **fonction** qui associe trois composants de couleur à un nom.

Le nom d'une couleur est déterminé par les règles suivantes, appliquées dans l'ordre. La première règle qui correspond à la couleur d'entrée doit être sélectionnée.

* La couleur `[0, 0, 0]` est "noir pur".
* La couleur `[255, 255, 255]` est "blanc pur".
* La couleur `[255, 0, 0]` est "rouge pur".
* La couleur `[0, 255, 0]` est "vert pur".
* La couleur `[0, 0, 255]` est "bleu pur".
* La couleur `[128, 128, 128]` est "gris parfait".
* Toute couleur dont tous les composants sont inférieurs à 31 est "presque noire".
* Toute couleur dont le composant rouge est supérieur à 128, dont les composants vert et bleu sont entre 0 et 127 est "rougeâtre".
* Toute couleur dont le composant vert est supérieur à 128, dont les composants rouge et bleu sont entre 0 et 127 est "verdâtre".
* Toute couleur dont le composant bleu est supérieur à 128, dont les composants rouge et vert sont entre 0 et 127 est "bleuté".
* Toute autre couleur est nommée "inconnue".

```rust
const fn color_name(color: &[u8; 3]) -> &str;
```

Vous devrez peut-être ajouter des annotations de *durée de vie* à la fonction pour qu'elle compile. Spécifiquement, le test suivant doit compiler et fonctionner :

```rust
#[cfg(test)]
#[test]
fn test_lifetimes() {
    let name_of_the_best_color;

    {
        let the_best_color = [42, 42, 42];
        name_of_the_best_color = color_name(&the_best_color);
    }

    assert_eq!(name_of_the_best_color, "unknown");
}
```

---

## Exercice 03 : Le Groupe le Plus Grand

```txt
Répertoire de rendu :
    ex03/

Fichiers à rendre :
    src/lib.rs  Cargo.toml

Symboles autorisés :
    <[u32]>::{len, is_empty, contains}
```

Écrivez une **fonction** qui retourne le plus grand sous-tableau de `haystack` contenant *tous* les nombres de `needle`.

```rust
fn largest_group(haystack: &[u32], needle: &[u32]) -> &[u32];
```

* Lorsque plusieurs groupes correspondent à `needle`, le plus grand est retourné.
* Lorsque plusieurs groupes les plus grands sont trouvés, le premier est retourné.

Exemple :

```rust
assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5], &[5, 5]));
assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[], &[]));
assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1], &[]));
```

Encore une fois, vous devrez peut-être spécifier des *annotations de durée de vie* pour la fonction. Pour vérifier si vos annotations sont correctes, vous pouvez utiliser ce test pré-défini `test_lifetimes`. Il doit compiler et s'exécuter.

```rust
#[test]
#[cfg(test)]
fn test_lifetimes() {
    let haystack = [1, 2, 3, 2, 1];
    let result;

    {
        let needle = [2, 3];
        result = largest_group(&haystack, &needle);
    }

    assert_eq!(result, &[2, 3, 2]);
}
```

---

## Exercice 04 : Des Boîtes dans des Boîtes

```txt
Répertoire de rendu :
    ex04/

Fichiers à rendre :
    src/lib.rs  Cargo.toml

Symboles autorisés :
    <[i32]>::{len, is_empty, swap}  std::{assert, assert_eq, panic}
```

Vous recevez une liste de boîtes (`[largeur, hauteur]`). Triez cette liste de boîtes de manière à ce que chaque boîte soit *contenue* dans la précédente. Si l'opération n'est pas possible, la fonction doit paniquer.

```rust
fn sort_boxes(boxes: &mut [[u32; 2]]);
```

Exemple :

```rust
let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
sort_boxes(&mut boxes);
assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
```

---

## Exercice 05 : Déduplication

```txt
Répertoire de rendu :
    ex05/

Fichiers à rendre :
    src/lib.rs  Cargo.toml

Symboles autorisés :
    std::vec::Vec::{remove, len}
```

Écrivez une **fonction** qui enlève tous les éléments répétées d'une liste, en préservant son ordre initial.

```rust
fn deduplicate(list: &mut Vec<i32>);
```

Exemple :

```rust
let mut v = vec![1, 2, 2, 3, 2, 4, 3];
deduplicate(&mut v);
assert_eq!(v, [1, 2, 3, 4]);
```

---

## Exercice 06 : Grande Addition

```txt
Répertoire de rendu :
    ex06/

Fichiers à rendre :
    src/lib.rs  Cargo.toml

Symboles autorisés :
    <[i32]>::{is_empty, len}
    std::vec::Vec::{push, len, is_empty, new, reverse}
    u8::is_ascii_digit
    std::assert
```

Écrivez une **fonction** qui additionne deux nombres. Les nombres sont donnés sous forme de liste de chiffres décimaux et peuvent être extrêmement grands.

```rust
fn big_add(a: &[u8], &[u8]) -> Vec<u8>;
```

* `a` et `b` ne doivent contenir que des chiffres (`b'0'` à `b'9'` inclus). Si un autre caractère est trouvé, la fonction doit paniquer.
* Si `a` ou `b` est vide, la fonction panique.
* Les nombres d'entrée peuvent contenir des zéros initiaux, mais le résultat ne doit pas en avoir.

Exemple :

```rust
assert_eq!(big_add(b"2", b"4"), b"6");
assert_eq!(big_add(b"0010", b"0200"), b"210");
```

---

## Exercice 07 : Justifie-toi !

```txt
Répertoire de rendu :
    ex07/

Fichiers à rendre :
    src/main.rs  Cargo.toml

Dépendances autorisées :
    ftkit  unicode-width(v0.1.10)

Symboles autorisés :
    ftkit::ARGS  ftkit::read_line
    unicode-width::UnicodeWidthStr
    std::vec::Vec::{new, push, clear}
    <[T]>::{len, is_empty}
    std::string::String::{new, as_str}
    str::{parse, trim, is_empty, len, split_whitespace, to_string}
    std::{eprintln, print, println}
    std::result::Result::unwrap
    std::{assert, assert_eq, panic}
```

Créez un **programme** qui prend en entrée un nombre de colonnes, et tente de justifier le texte qu'on lui fournit sur l'entrée standard autant que possible pour ce nombre de colonnes.

* L'entrée est séparée en "paragraphes". Chaque "paragraphe" est séparé par au moins deux retours à la ligne `'\n'`. La dernière ligne de chaque paragraphe n'est *pas* justifiée.
* Les espaces multiples dans l'entrée sont traités comme un seul.
* Dans le résultat final, les paragraphes sont toujours séparés par une seule ligne vide.
* Si un mot ne tient pas sur une seule ligne, il obtient sa propre ligne et ignore la contrainte des colonnes.
* Si l'utilisateur ne fournit aucun argument, trop d'arguments, ou si l'argument est invalide, le programme doit paniquer.
* Attention ! `é` a une largeur de `1`, mais prend plusieurs octets en mémoire !

Exemple :

```txt
>_ << EOF cargo run -- 20 | cat -e
Hey,         how   are
you?      Can
you hear me     screaming in your ears? 



I        don't!
EOF
Hey,  how  are  you?$  
Can   you   hear  me$ 
screaming   in  your$ 
ears?$ 
$ 
I don't!$ 
>_ << EOF cargo run -- 10
a
b
00000000000000000000000000000000000000000
c
d
EOF
a        b  
00000000000000000000000000000000000000000 
c d  
```