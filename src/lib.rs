//! agent-ai-harness : le harnais de confiance autour du code genere par IA.
//!
//! L'idee (entretien Quentin Adam / Clever Cloud, yt:AiytemqB_F0) :
//! le LLM ne comprend pas vraiment ce qu'il ecrit ; on ne lui fait pas confiance
//! a la main mais avec un SYSTEME de verification automatique qui compresse la
//! boucle de retroaction. Le code devient jetable ; ce qui compte ce sont les
//! specs, l'archi et les tests.

pub mod pricing;
