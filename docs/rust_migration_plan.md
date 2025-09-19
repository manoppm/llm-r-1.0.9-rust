Titre: Migration Rust-core riche (Rust-only)

Objectif:
- Passer progressivement le cœur du projet vers Rust-only tout en maintenant les interfaces existantes le temps de la migration.
- Remplacer le core par une API Rust enrichie: load_model, run_pipeline, save_state, avec tests.

Phases proposées:
1) Phase 1 – API enrichie et tests unitaires
   - Ajouter LlmEngine avec load_model, run_pipeline, save_state.
   - Ajouter tests unitaires pour chaque méthode.
   - Adapter CLI pour utiliser la nouvelle API.

2) Phase 2 – Migration progressive
   - Remplacer les composants externes (bindings Python/Node éventuels) par des appels Rust.
   - Ajouter tests d’intégration CLI -> core.

3) Phase 3 – Finalisation et CI Rust-only
   - Déprécation des anciens scripts Python/Node.
   - CI renforcé (linting, benchmarks si nécessaire).
