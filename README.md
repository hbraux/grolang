# GroLang

**_Le langage de programmation pour les grolandaises et grolandais._**

![Build](https://github.com/hbraux/grolang/workflows/maven/badge.svg)


**GroLang** (ou **Gro**) est un langage de programmation expérimental et à usage éducatif ; son objectif est de
proposer une (modeste) alternative à [python](https://www.python.org/) pour apprendre à coder, tout en comblant certaines de ses lacunes.
En particulier, les points fort de Gro sont :
* une syntaxe simple inspirée de Scala et Kotlin (pas d'indentation, séparateur optionnel)
* un interpréteur convivial (auto complétion, couleurs, débuggage)
* un typage fort
* le support de la programmation fonctionnelle

Quelques exemples:
```
print("hello world")
const PI = 3.14159
val p = 2*PI
fun fact(n: Int) = if (n <= 1) 1 else n*fact(n-1)
fact(10)
```


## Installation

> GroLang est encore **en développement** et même si l'interpréteur est opérationel, il n'est fourni qu'à titre de _sandbox_
(bas à sable).


## Guide

### Variables

Les mots clés `const`, `val` et `val` permettent de définir une constante globale, une variable immutable et
une variable mutable. On peut préciser le type de la variable ; sinon il est inféré automatiquement.

```
const PI = 3.14159
val perim : Float = PI * 2
var scoreTotal: Int = 1200
```

Le parser n'impose pas de règle de nommage particulière, mais les conventions sont les mêmes qu'en Java:
* constantes en majuscules
* variables en [camelCase](https://en.wikipedia.org/wiki/Camel_case) commençant par une minuscule
* types en camelCase commençant par une majuscule

### Types de base

Gro supporte les types `Int` (entier sur 8 bytes), `Float` (nombre décimal sur 8 bytes), `bool` (true/false),
`str` (chaine de caractères) :

```
val unEntier: Int = 123
val unEntierTresGrand = 123_456_789_000 # le séparateur _ améliore la lecture.
val unDecimal: Float = 123.45
val unDecimalAvecExposant = 1.23e45
val unBoolen: Bool = true
val uneChaine: Str = "hello"
val uneAutreChaine = "on peut echaper un \" en le prefixant avec \\."
```

### Collections et Structures

Gro supporte les types `List`, `Map` (dictionnaire) et permet de définir un type custom avec `Struct`.
Le type est optionnel et inféré à partir de la valeur.

```
val uneListeDentiers = [12, 4, 18, 16, 11]
val uneListeDeDecimaux: List<Float> = [ 1.23, 4.56 ] 
val uneMap: Map<String,Int> = { "paul": 12, "eric": 9 }

struct Point(x: Float, y: Float)
```

### Fonctions

Les opérateurs standards comme +, *, / >=, !=, ==, etc sont supportés en mode in-fixé naturel, comme en maths.
Par exemple `a + b`.

Mais on peut également appeler directement la fonction correspondante, ou bien la méthode sur le premier élément :
```
add(a, b)
a.add(b)
```

Dans les 3 cas, le parser produit la même expression :
```
> read("a + b")
add(a,b)
> read("a.add(b)")
add(a,b)
```

L'opérateur `fun` permet de définir une fonction. Les paramètres sont spécifiés avec leur type ; le type de
retour de la fonction est optionel. Le corps de la fonction est soit un block { .. }, soit une expression retournant
une valeur.
```
fun fact(n: Int) : Int = { if (n <= 1) 1 else n*fact(n-1) }
```

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

Beaucoup trop de choses. Toute contribution est la bienvenue
