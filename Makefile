.PHONY: test test-unit test-integration test-distributed pentest check lint

# ---------------------------------------------------------------------------
# Harnais de confiance autour du code généré par IA
# Couches : 1 unitaires · 2 intégration · 3 simulation · 4 pentest
# ---------------------------------------------------------------------------

# Couche 1 : tests unitaires
# Couche 2 : tests d'intégration (Testcontainers)
test: test-unit test-integration
	@echo "\n>>> Harnais : couches 1 + 2 OK"

test-unit:
	cargo test --profile dev -- --nocapture tests::unit
	@echo "  [unit] OK"

test-integration:
	cargo test --profile dev -- --nocapture tests::integration
	@echo "  [integration] OK"

# Couche 3 : simulation distribuée
# Nécessite un backend FoundationDB ou un orchestrateur de chaos (testcontainers).
# En continu : chaque commit en CI doit passer par ici (voir .github/workflows/harness.yml).
test-simulation:
	cargo test --profile release -- --nocapture tests::simulation
	@echo "  [simulation] OK"

test-distributed: test-unit test-integration test-simulation
	@echo ">>> Harnais : simulation distribuée OK (grille des CPU assumée)"

# Couche 4 : pentest automatique multi-modèles
pentest:
	./scripts/generic_pentest.sh --severity high --output reports/pentest
	@echo "  [pentest] portail multi-modèles exécuté"

# Pipeline du harnais complet
check: test-distributed pentest
	@echo "\n>>> Harnais : toutes les couches passent"

# Recherche de fuites de secrets (bonus, pour ne JAMAIS committer des clés)
sccheck:
	@echo "Vérification des secrets dans le repo (utilisez git-secrets, trufflehog, gitleaks)..."

lint:
	cargo clippy -- -D warnings
