# agent-ai-harness

> On ne paie plus les développeurs pour écrire du code — on paie pour les SPECS, l'architecture et les tests.

Le **harnais** (harness) est le système de garanties que l'on pose **autour** du code généré par les agents IA. Inspiré de l'entretien d'Underscore_ avec **Quentin Adam (Clever Cloud)** : *« On ne paie plus les développeurs pour écrire du code »* (https://youtu.be/AiytemqB_F0).

## L'idée

Un LLM génère du code trop vite pour qu'un humain relise et teste tout à la main ; et il ne comprend pas vraiment ce qu'il écrit. La seule façon de lui faire confiance, c'est de **compresser la boucle de rétroaction** : plus le système vérifie vite et automatiquement, plus l'agent peut corriger vite.

Ce qui devient « ton code », ce ne sont plus les fichiers sources, mais : **les specs, l'architecture, l'environnement de test, la matrice de vérification**.

## Architecture du harnais (5 couches)

| Couche | Type de test | Garantie | Outils de référence |
|---|---|---|---|
| 1 | **Unitaires** | Chaque portion de code donne un résultat prévisible | test runner, TDD |
| 2 | **Intégration** | Le code communique bien avec le monde extérieur (DB, API) | Testcontainers, mocks |
| 3 | **Simulation distribuée** | Le système tient sous contraintes chaotiques (perte réseau, crash RAM, redémarrage) | FoundationDB simulator, chaos |
| 4 | **Pentest automatique** | Aucune faille connue ne passe le portail | Pentest Claude + Gemini + Grok sur chaque commit |
| 5 | **Triage de bugs** | Un bug détecté devient une issue, dépilée et résolue par un agent | CI → issue → agent |

Voir [`docs/README.md`](docs/README.md) pour la conception détaillée.

## Démarrage rapide

```bash
# 1. Copier l'environnement
cp .env.example .env   # renseigner les clés

# 2. Vérifications de la chaîne (unitaires + intégration + lint)
make test

# 3. Simulation distribuée (nécessite backend FoundationDB ou testcontainers)
make test-distributed

# 4. Pentest automatique de la base de code
docker compose -f pixel/agent-codex.yaml up   # ou via GitHub Action sur chaque PR
```

## Plugins de confiance (boucles de rétroaction)

- [**pixel/agent-codex.yaml**](pixel/agent-codex.yaml) — config type Codex/Claude Code (gates + hooks).
- [**pixel/rust-flags.conf**](pixel/rust-flags.conf) — Réglages Rust stricts (le langage « qui se vibe-code le mieux »).
- [**.github/workflows/harness.yml**](.github/workflows/harness.yml) — CI : unit + integration + simulation + pentest.

## Structure

```
agent-ai-harness/
├── README.md
├── Makefile
├── .env.example
├── docs/
│   ├── README.md             # conception du harnais (5 couches)
│   └── meeting-prep-agent.md # la tâche « inhumaine » : préparation de réunion
├── src/
│   └── pricing.rs            # ex : moteur de tarification (cas éligible aux unitaires)
├── tests/
│   ├── unit.rs
│   ├── integration.rs
│   └── simulation/           # formalisme de simulation (garanties)
├── pixel/
│   ├── agent-codex.yaml
│   └── rust-flags.conf
├── scripts/
│   ├── run_tests.sh
│   └── generic_pentest.sh    # portail multi-modèles (Claude/Gemini/Grok)
└── .github/workflows/harness.yml
```

## Licence

MIT (sauf mention contraire). Ce dépôt est un gabarit : adaptez-le aux contraintes et critères de votre boîte.
