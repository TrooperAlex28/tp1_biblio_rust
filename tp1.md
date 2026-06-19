TP1 — Rust fondamental en ligne de commande

Contexte

Dans ce premier travail pratique, vous devez créer une petite application en ligne de commande avec Rust.

L’objectif n’est pas de créer une application graphique ni une application complète avec sauvegarde de données. Le but est plutôt de démontrer que vous êtes capables d’utiliser les notions fondamentales de Rust dans un programme structuré, lisible et fonctionnel.

Ce travail pratique permet de consolider les bases vues en classe, tout en intégrant certaines notions importantes du langage Rust : fonctions, traits, enums, pattern matching, chaînes de caractères, nombres, tuples et collections.

---

Sujet du TP

Vous devez créer une application en ligne de commande appelée :

Gestionnaire de bibliothèque

Votre programme doit permettre de gérer une petite collection de livres dans le terminal.

L’utilisateur doit pouvoir interagir avec l’application à partir d’un menu textuel.

Exemple de menu attendu :

```text
=== Gestionnaire de bibliothèque ===

1. Afficher tous les livres
2. Ajouter un livre
3. Rechercher un livre par titre
4. Modifier le statut d’un livre
5. Afficher les statistiques
6. Quitter
```

Les données n’ont pas besoin d’être sauvegardées dans un fichier. Il est donc normal que les livres ajoutés disparaissent lorsque le programme se termine.

---

Objectifs pédagogiques

À la fin de ce TP, vous devez démontrer que vous êtes capables de :

- déclarer et utiliser des variables ;
- manipuler des types numériques ;
- manipuler des chaînes de caractères ;
- créer et utiliser des fonctions ;
- utiliser des conditions ;
- utiliser des boucles ;
- utiliser des collections ;
- créer et utiliser des enums ;
- utiliser le pattern matching avec `match` ;
- utiliser au moins un tuple ;
- définir et utiliser au moins un trait simple ;
- organiser un programme Rust en ligne de commande.

---

Contraintes techniques

Votre projet doit respecter les contraintes suivantes :

- le projet doit être un projet Cargo valide ;
- le programme doit compiler sans erreur ;
- le programme doit s’exécuter avec la commande :

```bash
cargo run
```

- l’application doit fonctionner dans le terminal ;
- aucune interface graphique n’est demandée ;
- aucune base de données n’est demandée ;
- aucune sauvegarde dans un fichier n’est demandée ;
- aucune crate externe n’est nécessaire ;
- le code doit être écrit en Rust.

---

Structure minimale attendue

Votre programme doit représenter des livres.

Chaque livre devrait contenir au minimum les informations suivantes :

- un titre ;
- un auteur ;
- une année de publication ;
- un nombre de pages ;
- un genre ;
- un statut.

Exemple des informations possibles pour un livre :

```text
Titre : Le Petit Prince
Auteur : Antoine de Saint-Exupéry
Année : 1943
Pages : 96
Genre : Roman
Statut : Disponible
```

Vous pouvez choisir d’ajouter d’autres informations, mais ce n’est pas obligatoire.

---

Notions obligatoires à utiliser

1.  Fonctions

Votre code doit être organisé en plusieurs fonctions.

Par exemple, vous pouvez avoir des fonctions pour :

- afficher le menu ;
- afficher tous les livres ;
- ajouter un livre ;
- rechercher un livre ;
- modifier le statut d’un livre ;
- calculer les statistiques ;
- lire une entrée utilisateur.

Il ne faut pas mettre toute la logique uniquement dans la fonction `main`.

---

2.  Trait

Vous devez créer au moins un trait simple.

Par exemple, vous pouvez créer un trait permettant d’afficher proprement un élément :

```rust
trait Affichable {
    fn afficher(&self);
}
```

Vous devez ensuite utiliser ce trait dans votre programme.

Le nom du trait peut être différent. L’important est qu’il ait un rôle clair dans votre code.

---

3.  Enums et pattern matching

Vous devez créer au moins deux enums.

Par exemple :

```rust
enum Genre {
    Roman,
    Science,
    Informatique,
    Histoire,
    Autre,
}

enum Statut {
    Disponible,
    Emprunte,
}
```

Vous devez utiliser `match` pour traiter certains cas dans votre programme.

Par exemple, vous pouvez utiliser `match` pour :

- gérer les choix du menu ;
- afficher le genre d’un livre ;
- afficher le statut d’un livre ;
- modifier le statut d’un livre ;
- gérer les cas d’une recherche.

---

4.  Nombres

Votre programme doit manipuler des nombres.

Par exemple :

- l’année de publication ;
- le nombre de pages ;
- le nombre total de livres ;
- le nombre total de pages ;
- la moyenne du nombre de pages.

Vous devez faire au moins un calcul numérique dans votre application.

---

5.  Tuple

Votre programme doit utiliser au moins un tuple.

Par exemple, une fonction de statistiques pourrait retourner plusieurs valeurs :

```rust
(nombre_livres, total_pages, moyenne_pages)
```

Le tuple doit être utilisé de manière pertinente dans votre code.

---

6.  Collections

Votre programme doit utiliser une collection pour stocker les livres.

La collection attendue est un `Vec`.

Exemple :

```rust
let mut livres: Vec<Livre> = Vec::new();
```

Votre application doit permettre de parcourir cette collection pour afficher les livres, faire une recherche ou calculer des statistiques.

---

7.  Chaînes de caractères

Votre programme doit manipuler des chaînes de caractères.

Vous devrez utiliser des chaînes pour :

- les titres des livres ;
- les noms des auteurs ;
- les recherches effectuées par l’utilisateur ;
- les messages affichés dans le terminal.

Vous devrez donc utiliser des types comme `String` et/ou `&str`.

---

Fonctionnalités obligatoires

Votre application doit offrir les fonctionnalités suivantes.

1.  Afficher un menu principal

Le programme doit afficher un menu clair dans le terminal.

L’utilisateur doit pouvoir choisir une option.

Le menu doit se répéter tant que l’utilisateur ne choisit pas de quitter.

---

2.  Afficher tous les livres

Le programme doit afficher tous les livres présents dans la collection.

Chaque livre doit être affiché de manière lisible.

Si la collection est vide, le programme doit afficher un message clair.

---

3.  Ajouter un livre

L’utilisateur doit pouvoir ajouter un livre à la collection.

Le programme doit demander au minimum :

- le titre ;
- l’auteur ;
- l’année ;
- le nombre de pages ;
- le genre ;
- le statut.

Vous pouvez simplifier le choix du genre et du statut avec un petit menu.

---

4.  Rechercher un livre par titre

L’utilisateur doit pouvoir rechercher un livre à partir de son titre ou d’une partie de son titre.

Par exemple, si un livre s’appelle :

```text
Programmation Rust
```

Une recherche avec :

```text
Rust
```

devrait permettre de retrouver ce livre.

Si aucun livre ne correspond à la recherche, le programme doit afficher un message clair.

---

5.  Modifier le statut d’un livre

L’utilisateur doit pouvoir changer le statut d’un livre.

Par exemple :

- passer un livre de `Disponible` à `Emprunte` ;
- passer un livre de `Emprunte` à `Disponible`.

Vous pouvez demander à l’utilisateur de choisir le livre à modifier à partir de son numéro dans la liste.

---

6.  Afficher des statistiques

Votre programme doit afficher des statistiques simples sur la collection.

Les statistiques minimales attendues sont :

- le nombre total de livres ;
- le nombre total de pages ;
- la moyenne du nombre de pages ;
- le nombre de livres disponibles ;
- le nombre de livres empruntés.

Au moins une partie de ces statistiques doit être retournée par une fonction sous forme de tuple.

---

7.  Quitter proprement l’application

L’utilisateur doit pouvoir quitter l’application à partir du menu.

Le programme doit afficher un message de fin clair.

---

Données de départ

Votre programme peut commencer avec une collection vide.

Cependant, pour faciliter les tests, il est fortement recommandé d’ajouter quelques livres de départ dans votre code.

Par exemple, au lancement du programme, la collection pourrait déjà contenir trois livres.

Cela permet de tester immédiatement :

- l’affichage ;
- la recherche ;
- les statistiques ;
- la modification de statut.

---

Exemple d’exécution attendue

Voici un exemple simplifié du comportement attendu :

```text
=== Gestionnaire de bibliothèque ===

1. Afficher tous les livres
2. Ajouter un livre
3. Rechercher un livre par titre
4. Modifier le statut d’un livre
5. Afficher les statistiques
6. Quitter

Votre choix : 1

--- Liste des livres ---

1. Le Petit Prince
Auteur : Antoine de Saint-Exupéry
Année : 1943
Pages : 96
Genre : Roman
Statut : Disponible

2. Programmation Rust
Auteur : Équipe Rust
Année : 2021
Pages : 320
Genre : Informatique
Statut : Emprunté
```

---

Remise

Vous devez remettre votre projet Rust complet.

La remise doit contenir :

- le dossier du projet Cargo ;
- le fichier `src/main.rs` ;
- tout autre fichier nécessaire au bon fonctionnement du projet.

Le projet doit pouvoir être lancé avec :

```bash
cargo run
```

Assurez-vous de tester votre programme avant la remise.

---

Critères d’évaluation

Le TP est évalué sur 100 points.

| Critère                                                    |  Points |
| ---------------------------------------------------------- | ------: |
| Projet Cargo fonctionnel et programme qui compile          |      10 |
| Menu principal clair et boucle d’interaction fonctionnelle |      10 |
| Utilisation pertinente des fonctions                       |      10 |
| Utilisation correcte des chaînes de caractères             |      10 |
| Utilisation des nombres et de calculs simples              |      10 |
| Utilisation correcte d’une collection `Vec`                |      10 |
| Utilisation pertinente des enums                           |      10 |
| Utilisation du pattern matching avec `match`               |      10 |
| Utilisation pertinente d’un trait                          |      10 |
| Utilisation pertinente d’un tuple                          |       5 |
| Lisibilité, organisation et clarté générale du code        |       5 |
| **Total**                                                  | **100** |

---

Conseils

Commencez par créer une version simple de votre application.

Par exemple :

1. créez le projet Cargo ;
2. affichez un menu ;
3. ajoutez une boucle ;
4. créez une structure pour représenter un livre ;
5. ajoutez quelques livres de départ ;
6. affichez les livres ;
7. ajoutez ensuite la recherche ;
8. ajoutez la modification de statut ;
9. terminez avec les statistiques.

Il est préférable d’avoir une application simple qui fonctionne bien plutôt qu’une application plus complexe qui ne compile pas.

---

Rappel important

Votre code doit être compréhensible.

Vous devez être capable d’expliquer les choix que vous avez faits dans votre programme, notamment :

- pourquoi vous avez utilisé des fonctions ;
- à quoi servent vos enums ;
- où vous utilisez le pattern matching ;
- à quoi sert votre trait ;
- où vous utilisez un tuple ;
- où vous utilisez une collection ;
- comment les chaînes de caractères sont manipulées.

Le programme doit démontrer votre compréhension des bases de Rust.
