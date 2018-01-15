use keypair::PublicKey;

/// Account represents a single account in the Stellar network and its sequence
/// number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    public_key: PublicKey,
    sequence: u64,
}

impl Account {
    pub fn new(public_key: PublicKey, sequence: u64) -> Account {
        Account {
            public_key,
            sequence,
        }
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Increments the sequence number, returns the old sequence number.
    pub fn increment_sequence(&mut self) -> u64 {
        self.sequence += 1;
        self.sequence - 1
    }

    /// Returns the sequence number.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
}

#[cfg(test)]
mod tests {
    use super::Account;
    use keypair::PublicKey;

    #[test]
    fn test_increment_sequence() {
        let pk = PublicKey::from_account_id(
            "GCZHXL5HXQX5ABDM26LHYRCQZ5OJFHLOPLZX47WEBP3V2PF5AVFK2A5D",
        ).unwrap();
        let mut account = Account::new(pk, 999);
        let old = account.increment_sequence();
        assert_eq!(old, 999);
        assert_eq!(account.sequence(), 1000);
    }
}
