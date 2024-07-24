# Reinforcement Learning

## Contexte général

Ce projet a pour objectif d'implémenter manuellement des algorithmes de Deep Reinforcement Learning dans des environnements à recréer nous même, ou encore des environnements secrets données. 
Le but de ces environnements étants de tester ces algorithmes. 

Les algorithmes implémentés sont :
Dynamic Programming:
- Policy Iteration
- Value Iteration
Méthodes Monte Carlo:
- Monte Carlo ES
- On-policy first visit Monte Carlo Control
- Off-policy Monte Carlo Control
Temporal Difference Learning:
- Sarsa
- Q-Learning

Environnements implémentés :
- Line World
- Grid World
- Two round Rock Paper Scissors
- Monty Hall "paradox" level 1
- Secret Env 0
- Secret Env 1
- Secret Env 2

## Explication repo
Tout le code est inclus dans src/

Le dossier Env comporte l'entièreté des environnements développés, chaque environnement va avoir son propre algorithme.

Dans le main, une fonction run_all_and_save::run_all_and_save(); peut être utilisée afin de lancer l'ensemble des algorithmes sur chaque environnement.
Cette fonction va ensuite stocker les résultats dans results/results.csv, results/Pi_values (pour le stockage de modèles) et dans 2024-07-23 pour la moyenne des résultats par run

Il est possible de lancer un run manuellement pour chaque environnements/algo:
/// pour les secrets env
let mut env = Env::NomDeLEnvironnement::NomDeLEnvironnement::new();
let res = env.NomDelAlgorithme(paramètres en fonction de lalgo);
env.run_game_random_state_hashmap(res);
/// pour les autres
let mut env = Env::NomDeLEnvironnement::NomDeLEnvironnement::init();
let res = env.NomDelAlgorithme(paramètres en fonction de lalgo);
env.run_game_random_state_hashmap(res);

il existe 4 moyens de run un resultat d'entrainement: 
- run_game_hashmap si la sortie est un hashmap
- run_game_random_state_hashmap si la sortie est un hashmap et qu'on souhaite commencer dans un etat aléatoire
- run_game_random_hashmap si la sortie est un hashmap contenant un autre hashmap, ex Pi: HashMap<i32, HashMap<i32, f32>>
- run_game_vec pour un vecteur


Pour build :
cargo build --release

Pour lancer :
.\target\release\reinforcementlearning4A.exe

## Participants
Ce projet est développé et maintenu par :

Gabriel Bonjour
Badr Bouaissa
Clément Devarieux
