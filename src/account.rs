use keypair::{KeyPair, PublicKey};

/// Account represents a single account in the Stellar network and its sequence
/// number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    keypair: KeyPair,
    sequence: u64,
}

impl Account {
    pub fn new(keypair: KeyPair, sequence: u64) -> Account {
        Account { keypair, sequence }
    }

    pub fn keypair(&self) -> &KeyPair {
        &self.keypair
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.keypair.public_key()
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
    use keypair::KeyPair;

    #[test]
    fn test_increment_sequence() {
        let kp = KeyPair::random().unwrap();
        let mut account = Account::new(kp, 999);
        let seq = account.increment_sequence();
        assert_eq!(seq, 1000);
        assert_eq!(account.sequence(), 1000);
    }
}
