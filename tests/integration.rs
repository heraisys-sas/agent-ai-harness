//! Couche 2 du harnais : tests d'integration.
//! Verifient que le programme communique bien avec le monde exterieur (DB, API, tiers).
//! Moquez ou bootez des vrais logiciels configures avec Testcontainers.

// Exemple de structure (pseudo-Rust) :
//
// #[test]
// fn booking_roundtrips_through_postgres() {
//     let pg = testcontainers::clients::Cli::default().run(Postgres::default());
//     let port = pg.get_host_port_ipv4(5432);
//     let url = format!("postgres://postgres:postgres@127.0.0.1:{}/test", port);
//     let pool = PgPool::connect(&url).await.unwrap();
//     // spec : creer une commande, la relire, verifier les montants
// }
//
// Le MOCK (programme minimal qui se comporte comme le vrai, juste pour tester) :
// on simule par ex. une bourse pour tester un agent boursier sans dependre de
// donnees reelles -> inputs predictibles pour verifier le comportement.
