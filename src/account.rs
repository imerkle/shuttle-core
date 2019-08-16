use ed25519_dalek::PublicKey;

/// Account represents a single account in the Stellar network and its sequence
/// number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    account_id: PublicKey,
    sequence: u64,
}

impl Account {
    /// Create with `account_id` and `sequence` number.
    pub fn new(account_id: PublicKey, sequence: u64) -> Account {
        Account {
            account_id,
            sequence,
        }
    }

    /// Return the account public key.
    pub fn account_id(&self) -> &PublicKey {
        &self.account_id
    }

    /// Increments the sequence number, returns the old sequence number.
    pub fn increment_sequence(&mut self) -> u64 {
        self.sequence += 1;
        self.sequence
    }

    /// Returns the sequence number.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
}

#[cfg(test)]
mod tests {
    use super::Account;
    use ed25519_dalek::Keypair;
    use crypto;
    #[test]
    fn test_increment_sequence() {
        let seed = crypto::random_bytes(64);
        let kp = Keypair::from_bytes(&seed).unwrap();
        let mut account = Account::new(kp.public, 999);
        let seq = account.increment_sequence();
        assert_eq!(seq, 1000);
        assert_eq!(account.sequence(), 1000);
    }
}
