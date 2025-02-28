# Module 00 : Premiers Pas

## Préface

```rust
fn main() {
    break rust;
}
```

## Règles Générales

* Tout code que vous rendez doit compiler *sans avertissements* en utilisant le compilateur `rustc` disponible sur les machines de l'école, sans options supplémentaires. Vous n'êtes *pas* autorisé à utiliser du code `unsafe` n'importe où dans votre code (pas avant le dernier module :p).

* Pour les exercices utilisant le gestionnaire de paquets `cargo`, la même règle s'applique. Dans ce cas, seules les crates spécifiées dans la section `dépendances autorisées` sont permises. Toute autre dépendance est interdite. De manière plus générale, seuls les symboles spécifiés dans `symboles autorisés` sont autorisés dans un exercice.

* Vous n'êtes généralement *pas* autorisé à modifier les niveaux de lint - que ce soit en utilisant `#\[attributes\]`, `#!\[global_attributes\]` ou via des arguments en ligne de commande. Vous pouvez éventuellement autoriser le lint `dead_code` pour supprimer les avertissements concernant les variables, fonctions, etc., inutilisées.

```rust
// Soit globalement :
#![allow(dead_code)] 

// Soit localement, pour un simple élément :
#[allow(dead_code)]
fn ma_fonction_inutilisée() {}
```

## Exercice 00 : Hello, World!

```txt
dossier de rendu :
    ex00/

fichiers à rendre :
    hello.rs

symboles autorisés :
    std::println
```

Qu'est-ce qu'un programme sans effets secondaires ?

Créez un **programme** qui affiche la chaîne de caractères `Hello, World!`, suivie d'un saut de ligne.

```txt
>_ ./hello
Hello, World!
```

## Exercice 01 : Point de Non-Retour

```txt
dossier de rendu :
    ex01/

fichiers à rendre :
    min.rs

symboles autorisés :
    std::println
```

Créez une **fonction** `min` qui prend deux entiers et retourne le plus petit des deux. Pour que le fichier compile et soit testable, vous devez ajouter une fonction `main` démontrant que votre fonction est correcte.

La fonction doit être définie comme suit :

```rust
fn min(a: i32, b: i32) -> i32;
```

Oh, j'ai presque oublié. Le mot-clé `return` est interdit dans cet exercice ! Bonne chance avec ça ~

## Exercice 02 : yyyyyyyyyyyyyy

```txt
dossier de rendu :
    ex02/

fichiers à rendre :
    yes.rs  collatz.rs  print_bytes.rs

symboles autorisés :
    std::println  str::bytes
```

Créez trois **fonctions**. Chaque fonction doit utiliser un type de boucle différent pris en charge par Rust, et vous ne pouvez pas utiliser le même type de boucle deux fois.

Les fonctions doivent être définies comme suit :

```rust
fn yes() -> !;
fn collatz(start: u32);
fn print_bytes(s: &str);
```

La fonction `yes` doit afficher le message `y`, suivi d'un saut de ligne, indéfiniment.

```txt
y
y
y
y
y
y
y
...
```

La fonction `collatz` doit exécuter l'algorithme suivant...

* Soit *n* un nombre naturel.
* Si *n* est pair, alors *n* devient *n*/2.
* Si *n* est impair, alors *n* devient 3*n* + 1.

...jusqu'à ce que *n* soit égal à 1. À chaque itération, *n* doit être affiché sur la sortie standard, suivi d'un saut de ligne.

```txt
Entrée :
3

Sortie :
3
10
5
16
8
4
2
1
```

La fonction `print_bytes` doit afficher chaque octet de la chaîne fournie.

```txt
Entrée :
"Déjà Vu\n"

Sortie :
68
195
169
106
195
160
32
86
117
10
```

Encore une fois, vous devez ajouter des fonctions `main` pour prouver que vos fonctions sont correctes.

## Exercice 03 : FizzBuzz

```txt
dossier de rendu :
    ex03/

fichiers à rendre :
    fizzbuzz.rs

symboles autorisés :
    std::println
```

Créez un **programme** qui joue au célèbre (et adoré !) jeu "Fizz Buzz" de 1 à 100.

Les règles ont un peu changé, cependant. Elles doivent être suivies dans cet ordre :

* Lorsque le nombre est à la fois un multiple de 3 et de 5, "fizzbuzz" doit être affiché.
* Lorsque le nombre est un multiple de 3, "fizz" doit être affiché.
* Lorsque le nombre est un multiple de 5, "buzz" doit être affiché.
* Lorsque le nombre est congruent à 3 modulo 11, "FIZZ" est affiché.
* Lorsque le nombre est congruent à 5 modulo 11, "BUZZ" est affiché.
* Sinon, le nombre lui-même est affiché.

Exemple :

```txt
>_ ./fizzbuzz
1
2
fizz
4
buzz
fizz
7
8
fizz
buzz
11
fizz
13
FIZZ
fizzbuzz
BUZZ
17
fizz
19
buzz
...
```

Pour cet exercice, vous ne pouvez utiliser qu'une seule boucle `for` et une seule instruction `match`. Rien de plus.

## Exercice 04 : Expédition avec Cargo

```txt
dossier de rendu :
    ex04/

fichiers à rendre :
    src/main.rs  src/overflow.rs  src/other.rs  Cargo.toml

symboles autorisés :
    std::println
```

Créez un projet Cargo.

* Son nom doit être "module00-ex04"
* Il doit utiliser l’édition Rust 2021
* Son auteur doit être vous.
* Sa description doit être "ma réponse au cinquième exercice du premier module de la Piscine Rust de 42"
* Il ne doit pas être possible de publier le package, même en utilisant `cargo publish`.

* Les commandes suivantes doivent donner ce résultat :

```txt
>_ cargo run
Hello, Cargo!
>_ cargo run --bin other
Hey! I'm the other bin target!
>_ cargo run --release --bin other
Hey! I'm the other bin target!
I'm in release mode!
```

* En mode `release`, le binaire final ne doit avoir aucun symbole visible dans son exécutable.

```txt
>_ cargo build
>_ nm <target-dir>/debug/module00-ex04 | head
000000000004d008 V DW.ref.rust_eh_personality
0000000000049acc r GCC_except_table0
0000000000049ad8 r GCC_except_table1
00000000000493e0 r GCC_except_table1049
00000000000493f8 r GCC_except_table1051
0000000000049410 r GCC_except_table1060
0000000000049430 r GCC_except_table1072
0000000000049440 r GCC_except_table1073
000000000004945c r GCC_except_table1075
0000000000049470 r GCC_except_table1076
>_ cargo build --release
>_ nm <target-dir>/release/module00-ex04
nm: <target-dir>/release/module00-ex04: no symbols
```

* Il doit avoir un profil personnalisé héritant du profil "dev". Ce profil doit simplement désactiver les vérifications de dépassement d’entiers. Pour cette raison, vous le nommerez `no-overflows`.

```txt
>_ cargo run --bin test-overflows
thread 'main' panicked at 'attempt to add with overflow', src/overflow.rs:3:5
>_ cargo run --profile no-overflows --bin test-overflows
255u8 + 1u8 == 0
```

---

## Exercice 05 : Vendredi 13

```txt
dossier de rendu :
    ex05/

fichiers à rendre :
    src/main.rs  Cargo.toml

symboles autorisés :
    std::{assert, assert_eq, assert_ne}  std::panic  std::{print, println}
```

Écrivez un **programme** qui affiche chaque vendredi tombant le 13 du mois, depuis le premier jour de l'an 1 (qui était un lundi).

Pour accomplir cette tâche, vous devez écrire les fonctions suivantes :

```rust
fn is_leap_year(year: u32) -> bool;
fn num_days_in_month(year: u32, month: u32) -> u32;
```

* `is_leap_year` doit déterminer si une année donnée est bissextile ou non.
* `num_days_in_month` doit calculer combien de jours un mois donné d'une année donnée possède.

Exemple :

```txt
>_ cargo run
Friday, April 13, 1
Friday, July 13, 1
Friday, September 13, 2
Friday, December 13, 2
Friday, June 13, 3
Friday, February 13, 4
Friday, August 13, 4
Friday, May 13, 5
Friday, January 13, 6
Friday, October 13, 6
...
```

Vous devez ajouter des tests à votre projet Cargo pour vérifier que `is_leap_year` et `num_days_in_month` fonctionnent correctement. Plus précisément, vous devez prouver que :

* 1600 est une année bissextile.
* 1500 est une année commune.
* 2004 est une année bissextile.
* 2003 est une année commune.
* Février a 29 jours les années bissextiles, mais 28 les années communes.
* Les autres mois ont le bon nombre de jours, quelle que soit l’année.
* Passer un mois invalide à `num_days_in_month` doit faire paniquer la fonction.
* Passer l’année `0` à `is_leap_year` doit faire paniquer la fonction.

Ces tests doivent pouvoir être exécutés avec `cargo test`.

---

## Exercice 06 : Jeu de devinette

```txt
dossier de rendu :
    ex06/

fichiers à rendre :
    src/main.rs  Cargo.toml

dépendances autorisées :
    ftkit

symboles autorisés :
    ftkit::read_number  ftkit::random_number
    i32::cmp  std::cmp::Ordering
```

Créez un **programme** qui joue au jeu de la devinette.

```txt
>_ cargo run
Moi et ma sagesse infinie avons trouvé un secret digne de votre quête.
12
Cet étudiant n'est peut-être pas aussi intelligent qu'on me l'a dit. Cette réponse est évidemment trop faible.
25
Parfois, je me demande si je devrais prendre ma retraite. J’aurais deviné plus haut.
19
C'est exact ! Le secret était bien le nombre 19, que vous avez brillamment découvert !
```

Vous ne pouvez pas utiliser les opérateurs `<`, `>`, `<=`, `>=` et `==` !

---

## Exercice 07 : Comparaison de motifs de chaîne

```txt
dossier de rendu :
    ex07/

fichiers à rendre :
    src/lib.rs  src/main.rs  Cargo.toml

dépendances autorisées :
    ftkit

symboles autorisés :
    std::{assert, assert_eq}  <[u8]>::{len, is_empty}
    str::as_bytes  ftkit::ARGS
```

Créez une **bibliothèque** qui expose la fonction `strpcmp`.

```rust
fn strpcmp(query: &[u8], pattern: &[u8]) -> bool;
```

* `strpcmp` détermine si `query` correspond au `pattern` donné.
* `pattern` peut éventuellement contenir des caractères `"*"`, qui peuvent correspondre à n’importe quel nombre de caractères quelconques dans la chaîne de requête.

Exemple :

```rust
assert!(strpcmp(b"abc", b"abc"));

assert!(strpcmp(b"abcd", b"ab*"));
assert!(!strpcmp(b"cab", b"ab*"));

assert!(strpcmp(b"dcab", b"*ab"));
assert!(!strpcmp(b"abc", b"*ab"));

assert!(strpcmp(b"ab000cd", b"ab*cd"));
assert!(strpcmp(b"abcd", b"ab*cd"));

assert!(strpcmp(b"", b"****"));
```

Votre crate doit également inclure une cible binaire permettant de tester la bibliothèque facilement. Notez que la fonction `strpcmp` doit rester dans la *bibliothèque* !

```txt
>_ cargo run -- 'abcde' 'ab*'
yes
>_ cargo run -- 'abcde' 'ab*ef'
no
```