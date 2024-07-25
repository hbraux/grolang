# GroLang

![Build](https://github.com/hbraux/grolang/workflows/maven/badge.svg)


GroLang est un langage de programmation expérimental et à usage éducatif ; son objectif est de proposer aux débutants une 
alternative à [python](https://www.python.org/) pour apprendre à coder, tout en ne faisant pas l'impasse
sur des concepts clés comme le typage fort, l'immutabilité, la programmation fonctionelle et la gestion explicite des
optionels et des exceptions.

GroLang est un langage interprété et fournit un interpréteur (*REPL*) flexible permettant de construire son code par 
étapes, en se passant d'un éditeur de texte. La syntaxe, inspirée de Scala, Kolin, Python et Lisp, se veut la plus simple
possible et facile à retenir. Elle est également souple et permet d'appeler une fonction soit de manière classique F(x) ou 
bien par méthode x.F()

Quelques exemples de code:

```
print("hello world!")

val PI = 3.14159
val circ = PI * 2.0

def fact(n: Int) = if (n <= 1) 1 else n*fact(n-1)
fact(5)

val notes = [12 4 18 16 11 9]
val moy = notes.sum()/notes.size() # ou bien sum(notes)/size(notes)
```


## Installation

***Ce projet est en cours de développement et n'a pas encore été releasé***


## Guide



## Développement

