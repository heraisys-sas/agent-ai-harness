# Conception du harnais

Ce document traduit en système l'entretien de Quentin Adam (Clever Cloud) sur la façon dont son équipe encadre le code généré par IA.

## Principes

1. **Un LLM reste un LLM.** Il ne « comprend » pas le code : il prédit des tokens. Il est très bon pour proposer, médiocre pour garantir.
2. **La qualité vient de l'extérieur.** Ce n'est pas le modèle qui garantit, c'est le **système de vérification automatique** qui l'entoure.
3. **Compresser la boucle de rétroaction.** Plus vite un problème est détecté, plus vite l'agent corrige, meilleur est le résultat final — comme AlphaGo qui joue des millions de parties.
4. **Des langages stricts.** Plus le compilateur est exigeant (types forts, gestion mémoire, ex. Rust), meilleure est la boucle feedback modèle↔compilateur. JavaScript/Go compilent « relax » et reportent la faute au runtime.
5. **La spec est le contrat.** Donner à l'agent des spécifications claires donne les meilleurs résultats. Le code généré est jetable ; les specs et les tests sont la vraie valeur.

## Les 5 couches de garantie

### 1. Tests unitaires

Vérifier qu'une portion de code donne un **résultat prévisible et intelligible** dans différentes situations.

- Éligible : tout calcul métier (ex. moteur de tarification, `src/pricing.rs`).
- Rôle avec les agents : donner la spec + les tests d'abord, puis demander le code « **tant que les tests ne passent pas, recommence** ».
- Le développeur n'est plus le rédacteur du code : il est **spécificateur + testeur**. C'est « ce qu'on te demandait depuis des années » (TDD) exécuté automatiquement.

### 2. Tests d'intégration

Vérifier que le programme **communique bien avec le monde extérieur** : base de données, API, tiers.

- **Testcontainers** : booter des images réelles (Postgres, Redis…) pour tester les échanges.
- **Mocks** : un programme minimal qui se comporte comme le vrai, uniquement pour tester (ex. simuler une bourse pour tester un agent boursier).
- Avant : construire des mocks coûtait cher, on arbitrait. Maintenant : ça coûte moins cher que de ne pas le faire et ça **réduit la boucle**, donc c'est un gain net.

### 3. Simulation distribuée

Pour les **systèmes distribués** (ex. 960 machines), les bugs ne sont pas visibles sur une instance : ils viennent de la **distribution** (perte de 20 % des paquets, RAM qui crève, machine qui redémarre, partage de cluster…).

- Inspiré du **simulateur FoundationDB** : un simulateur secoue le système avec de l'aléatoire et énumère les cas.
- **Simulation formelle** : des garanties mathématiques sur les cas bizarres (une branche entière des maths), testée à **chaque commit** (coûteux en CPU, assumé).
- Sortie : un **rapport de bug** posté en **issue**, dépilée par un **autre agent** qui va la résoudre.

### 4. Pentest automatique

Tout commit entrant se tape un **pentest**. Idée clé : « si le pentest de *mon* Claude ne détecte pas un truc, il y a peu de chance que le pentest du Claude du mec qui essaie de me pénétrer le détecte ». Ensuite : lancer **Gemini, puis Grok, puis tout le monde**, en permanence, sur son propre système.

- Portail `scripts/generic_pentest.sh` : lance plusieurs modèles sur la base de code.
- Depuis qu'ils ont lancé l'IA « en free range » sur leurs installations, ils ont trouvé **plein de failles**.

### 5. Triage de bugs par agents

Quand la simulation ou le pentest détecte un problème :

- un **rapport de bug** est écrit automatiquement ;
- posté en **issue** ;
- **dépilé par un agent dédié** qui va le résoudre.

## La tâche « inhumaine » : préparation de réunion

Autre usage clé de l'entretien : ne pas demander à l'IA des tâches de « junior ». Lui demander des **tâches inhumaines**.

Exemple : le monolithe de Clever Cloud, ~30 personnes par semaine, réunion de synchro le lundi inefficace. Au lieu de faire faire du code, on lui demande :

> « Prends toutes les branches en cours, les résultats des tests, compare-les. Repère où les tests unitaires chient, ce qui a l'air difficile, ce qui va bien, les commentaires PR/issues où les gens s'engueulent, les décisions d'archi à prendre. Lis les mails, Slack, pour trouver les points de tension. Produis un document de préparation de réunion hyper qualifié. »

C'est **3-4 jours de travail documentaire** pour un humain → **20 minutes d'exécution IA**, et la réunion devient efficace. Voir [`meeting-prep-agent.md`](meeting-prep-agent.md) pour un prompt réutilisable.

## Langage recommandé : Rust

Dans l'entretien : Rust « se vibe-code hyper bien » car :

- pas de changement cassant (langage ~10-15 ans, uniforme) ;
- dataset propre mais pas « pollué » par 120 itérations (vs Java 8 vs Java 21) ;
- compilateur très exigeant → **la meilleure boucle de rétroaction modèle↔compilateur**.

Réglages stricts fournis : [`pixel/rust-flags.conf`](../pixel/rust-flags.conf).

## Coût : des tokens brûlés en test, pas en code

On regarde souvent les tokens générateurs. Le vrai budget, c'est le **garde-fou** : `tokens de génération` sont garnis par `tokens brûlés pour vérifier`. On n'écrira **jamais plus** ce code à la main — ces derniers ne coûtent rien en maintenance future. Le code devient **jetable**, encadré par specs + archi + test env.

## Culture (leçons de l'entretien)

- Les mecs qui ont changé le boulot de tout le monde sont les plus résistants. **Convaincre un par un**, les tech leads, y compris physiquement.
- Ne pas imposer un outil top-down : ça avance trop vite. Onglet « notes de frais » : *« achetez ce que vous voulez, testez, partagez — interdit les plans annuels »*.
- Ce sont les **seniors** qui s'adaptent le plus vite : ils n'ont plus le temps de coder, et là ils peuvent coder + gérer, avec du code « jamais aussi propre ».
- Notre métier a **changé** : le coût du logiciel va baisser, l'exigence de qualité va devenir terrible. Suivre n'est plus un choix mais une nécessité compétitive.
