# GroLang

**_Le langage de programmation pour les grolandaises et grolandais._**

![Build](https://github.com/hbraux/grolang/workflows/maven/badge.svg)


**GroLang** (ou **Gro**) est un langage de programmation expérimental et à usage éducatif ; son objectif est de
proposer une (modeste) alternative à [python](https://www.python.org/) pour apprendre à coder, tout en comblant certaines de ses lacunes.
En particulier, les points fort de Gro sont :
* une syntaxe simple (pas d'indentation, séparateur optionel)
* un interpréteur convivial (auto complétion, couleurs)
* un typage fort
* des objets immutables
* la gestion des optionnels (à venir)
* le support de la programmation fonctionnelle (à venir)


Quelques exemples de code:

```
print("hello world!")

const PI = 3.14159
val circ = PI * 2.0

def fact(n: Int) = if (n <= 1) 1 else n*fact(n-1)
fact(5)

val notes = [12 4 18 16 11 9]
val moy = notes.sum()/notes.size() # ou bien sum(notes)/size(notes)
```


## Installation

> GroLang est encore **en développement** et même si l'interpréteur est opérationel, il n'est fourni qu'à titre de _sandbox_
(bas à sable).


## Guide

### Variables

à documenter

### Types de base
à documenter

### Collections
à documenter

### Fonctions

à documenter

## Développement

GroLang est développé en [rust](https://www.rust-lang.org/) et utilise la librairie [pest](https://pest.rs/) pour
le parsing. À noter que le code est loin d'étre parfait vu que c'est mon premier projet en rust, et qu'il n'est pas
du tout optimisé à ce stade (beaucoup de `clone(..)` évitables je pense)

Les grandes lignes du code :
* L'interpréteur utilise une boucle _read-eval-print_ ([repl](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop))
* Le parser transforme une chaine de caractères (String) en une **expression** (enum `Expr`) dont le type dépend de
  la règle syntaxique. Le parser est complètement agnostique de la sémantique et ne vérifie que la syntaxe.
* Les expressions sont évaluées de facon recursive et retournent soit une autre expression, soit une exception qui
  est encapsulée dans une expression de type `Failure`.
* L'objet `scope` permet d'assurer la persistence des variables et définitions. Un `scope` "fils" est créé à chaque
  appel de fonction et enrichi avec les arguments de la fonction.
* Les arguments des fonctions sont évaluées avant l'appel de la fonction. Une fonction ne peut PAS modifier son scope.
* Les arguments des **macros** (mot-clés comme `var`, `fun`, `struct`) sont évaluées par la macro de façon "paresseuse"
  (lazy). Seule une macro peut modifier le scope, par exemple en déclarant une nouvelle variable


**Reste à faire**

Trop de choses (voir les TODOs dans le code). Toute contribution est la bienvenue
