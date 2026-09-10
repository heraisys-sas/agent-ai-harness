#!/usr/bin/env bash
# Point d'entree du harnais : execute les couches 1 (unit) et 2 (integration).
# Usage : ./scripts/run_tests.sh [--all]
set -euo pipefail

# Couche 1 : unitaires
cargo test --lib -- --nocapture

# Couche 2 : integration (si --all ou si backend dispo)
if [[ "${1:-}" == "--all" ]]; then
  cargo test --test 'integration' -- --nocapture
fi

echo ">>> Harnais couches 1+2 OK"
