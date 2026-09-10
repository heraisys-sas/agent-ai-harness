# Couche 3 : simulation distribuee

Pour les systemes distribues, les bugs ne sont pas visibles sur une instance : ils
viennent de la distribution (perte de paquets, RAM qui creve, machine qui redemarre,
partage de cluster...).

Comme le simulateur FoundationDB : on « secoue » le systeme avec de l'aleatoire pour
enumere les cas et detecter les bugs qui n'apparaissent qu'a l'echelle.

## Principes

- **Simulation formelle** : des garanties mathematiques sur les cas bizarres (un
  domaine entier des maths). Testee a CHAQUE commit via la CI (voir .github/workflows).
- **Cout assume** : ca grille des c[plectrons] de CPU. Associe au budget « tokens de
  verification » du harnais.
- **Sortie** : un rapport de bug ecrit automatiquement, poste en issue, depilee par un
  agent dedie qui resout le probleme (couche 5).

## Scenarios type

- 20 % de paquets reseau perdus sur un noeud.
- Arret puis relance d'une machine.
- 30 % d'ecritures en memoire ralenties (barrette RAM en train de crever).
- Partage de cluster deconnecte.
- Election de leader contestee, split brain.

## Variables d'environnement

- `FDB_CLUSTER_FILE` : fichier cluster FoundationDB.
- `SIMULATION_ITERATIONS` : nombre d'iterations aleatoires (defaut 10000).
