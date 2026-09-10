# Agent de préparation de réunion

Le cas d'usage « inhumain » de l'entretien : ne pas demander à l'IA une tâche de junior, mais une **tâche que seul un système automatisé peut faire** — ici, synthétiser plusieurs jours de documentation éparpillée en un document de préparation de réunion utilisable.

> Pourquoi : la réunion de synchro d'un gros monolithe (~30 personnes par semaine) est inefficace car chacun parle de sa brique sans réel sujet commun. Le travail documentaire pour identifier les vrais points de tension prend 3-4 jours à un humain → 20 minutes à l'IA.

## Prompt (à adapter)

```
Tu es l'assistant de préparation de réunion d'équipe.

Prends toutes les branches de code sur lesquelles les gens bossent et les résultats de tests, et compare-les. Essaie de voir :

1. Où les tests unitaires échouent les uns après les autres.
2. Ce qui a l'air difficile (beaucoup d'échecs, de conflits).
3. Ce qui se passe bien.
4. Les commentaires dans les pull requests et issues où il y a beaucoup d'échanges d'un point de vue de tension : les gens ont l'air de s'engueuler ou pas ?
5. Y a-t-il une décision d'architecture à prendre, manifestement en cours par écrit, où les esprits s'échauffent (alors qu'en réunion ça irait mieux) ?

Lis aussi mes mails et Slack pour comprendre où sont les points de tension et d'incompréhension entre les gens.

Produis un document de préparation de la réunion hyper qualifié, qui identifie les sujets réellement intéressants pour les participants, hiérarchise par urgence / risque, et signale les sujets qui méritent un temps de discussion en présentiel.
```

## Ce qui le rend efficace

- On lui demande ce qu'**un humain ne ferait pas** (défricher 3-4 jours de documents avant une réunion hebdomadaire n'a aucun sens) — c'est la définition d'une « tâche inhumaine ».
- La sortie est un **document structuré et priorisé** qu'on contrôle, pas du code à relire.
- Il déplace la valeur : la réunion sert à **trancher**, plus à **synchroniser**. C'est le même déplacement que pour le code (specs/tests plutôt que code).
