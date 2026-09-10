//! Moteur de tarification - exemple de code eligible aux tests unitaires.
//! Conformement a l'entretien : ce code n'est pas cense etre relu a la main.
//! La vraie valeur, ce sont les SPEC + tests. Le dev devient specificateur + testeur,
//! et l'agent produit le code jusqu'a ce que les tests passent.

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteLine {
    pub product: String,
    pub quantity: u32,
    pub unit_price_cents: i64,
    pub vat_rate_bps: u16, // taux en points de base (20 pct = 2000)
}

impl QuoteLine {
    /// Montant HT en centimes, sature pour eviter l'overflow.
    pub fn net_cents(&self) -> i64 {
        (self.quantity as i64).saturating_mul(self.unit_price_cents)
    }

    /// Montant TTC en centimes.
    pub fn gross_cents(&self) -> i64 {
        let net = self.net_cents();
        net.saturating_mul(10000 + i64::from(self.vat_rate_bps)).div_euclid(10000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_is_quantity_times_price() {
        let line = QuoteLine {
            product: "instance.xsmall".into(),
            quantity: 3,
            unit_price_cents: 100,
            vat_rate_bps: 2000,
        };
        assert_eq!(line.net_cents(), 300);
    }

    #[test]
    fn gross_applies_vat() {
        let line = QuoteLine {
            product: "instance.xsmall".into(),
            quantity: 1,
            unit_price_cents: 100,
            vat_rate_bps: 2000,
        };
        assert_eq!(line.gross_cents(), 120);
    }

    #[test]
    fn large_values_do_not_overflow() {
        let line = QuoteLine {
            product: "big".into(),
            quantity: u32::MAX,
            unit_price_cents: i64::MAX,
            vat_rate_bps: 0,
        };
        assert_eq!(line.net_cents(), i64::MAX);
    }
}
