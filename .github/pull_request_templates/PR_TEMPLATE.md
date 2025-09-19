Titre du PR: [Courte description]

Sections:
- Résumé
- Contexte et motivation
- Changements apportés (API/Core)
- Tests ajoutés / vérifications manuelles
- Migration plan (étapes prévues pour passer de l’existant vers Rust-only)
- Impact sur les dépendances et le déploiement
- Contraintes et risques connus
- Comment tester localement

Vérifications reques:
- cargo test --all
- cargo clippy --all -- -D warnings
- build de CLI si applicable
