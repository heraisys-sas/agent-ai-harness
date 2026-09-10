//! Couche 1 du harnais : tests unitaires.
//! Verifie que chaque portion de code produit un resultat previsible et intelligible.

use agent_ai_harness::pricing::QuoteLine;

#[test]
fn quote_net_cents() {
    let line = QuoteLine {
        product: "instance.xsmall".into(),
        quantity: 2,
        unit_price_cents: 250,
        vat_rate_bps: 2000,
    };
    assert_eq!(line.net_cents(), 500);
    assert_eq!(line.gross_cents(), 600);
}

#[test]
fn zero_quantity_zero_amount() {
    let line = QuoteLine {
        product: "bw".into(),
        quantity: 0,
        unit_price_cents: 10_000,
        vat_rate_bps: 2000,
    };
    assert_eq!(line.net_cents(), 0);
    assert_eq!(line.gross_cents(), 0);
}

#[test]
fn vat_rounding_is_banker_safe() {
    let line = QuoteLine {
        product: "x".into(),
        quantity: 1,
        unit_price_cents: 1,
        vat_rate_bps: 1960,
    };
    // 1 * 1.196 = 1.196 -> div_euclid(10000) = 1 centime
    assert_eq!(line.gross_cents(), 1);
}
