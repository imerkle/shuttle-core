use crypto;

const PUBLIC_PASSPHRASE: &str = "Public Global Stellar Network ; September 2015";
const TEST_PASSPHRASE: &str = "Test SDF Network ; September 2015";

/// A Stellar Network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Network {
    passphrase: String,
}

impl Network {
    pub fn new(passphrase: String) -> Network {
        Network { passphrase }
    }

    pub fn public_network() -> Network {
        Self::new(PUBLIC_PASSPHRASE.to_string())
    }

    pub fn test_network() -> Network {
        Self::new(TEST_PASSPHRASE.to_string())
    }

    pub fn passphrase(&self) -> &str {
        &self.passphrase
    }

    pub fn network_id(&self) -> Vec<u8> {
        crypto::hash(self.passphrase.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use Network;

    #[test]
    fn test_public_network_id() {
        let network = Network::public_network();
        let id = network.network_id();
        let expected_id = vec![
            0x7A, 0xC3, 0x39, 0x97, 0x54, 0x4E, 0x31, 0x75, 0xD2, 0x66, 0xBD, 0x02, 0x24, 0x39,
            0xB2, 0x2C, 0xDB, 0x16, 0x50, 0x8C, 0x01, 0x16, 0x3F, 0x26, 0xE5, 0xCB, 0x2A, 0x3E,
            0x10, 0x45, 0xA9, 0x79,
        ];
        assert_eq!(id, expected_id);
    }

    #[test]
    fn test_test_network_id() {
        let network = Network::test_network();
        let id = network.network_id();
        let expected_id = vec![
            0xCE, 0xE0, 0x30, 0x2D, 0x59, 0x84, 0x4D, 0x32, 0xBD, 0xCA, 0x91, 0x5C, 0x82, 0x03,
            0xDD, 0x44, 0xB3, 0x3F, 0xBB, 0x7E, 0xDC, 0x19, 0x05, 0x1E, 0xA3, 0x7A, 0xBE, 0xDF,
            0x28, 0xEC, 0xD4, 0x72,
        ];
        assert_eq!(id, expected_id);
    }
}
