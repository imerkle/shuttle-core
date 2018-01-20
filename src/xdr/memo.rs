use serde_xdr;
use error::Result;
use xdr::{FromXdr, ToXdr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Memo {
    None,
    Text(String),
    Id(u64),
    Hash(MemoHash),
    Return(MemoHash),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoHash {
    #[serde(with = "serde_xdr::opaque_data::fixed_length")] pub buf: [u8; 32],
}

impl ToXdr<Memo> for ::Memo {
    fn to_xdr(&self) -> Result<Memo> {
        match *self {
            ::Memo::None => Ok(Memo::None),
            ::Memo::Id(id) => Ok(Memo::Id(id)),
            ::Memo::Text(ref s) => Ok(Memo::Text(s.clone())),
            ::Memo::Hash(buf) => Ok(Memo::Hash(MemoHash { buf })),
            ::Memo::Return(buf) => Ok(Memo::Return(MemoHash { buf })),
        }
    }
}

impl<'de> FromXdr<'de, Memo> for ::Memo {
    fn from_xdr(memo: Memo) -> Result<::Memo> {
        match memo {
            Memo::None => Ok(::Memo::None),
            Memo::Id(id) => Ok(::Memo::Id(id)),
            Memo::Text(s) => Ok(::Memo::Text(s)),
            Memo::Hash(MemoHash { buf }) => Ok(::Memo::Hash(buf)),
            Memo::Return(MemoHash { buf }) => Ok(::Memo::Return(buf)),
        }
    }
}

#[cfg(test)]
mod tests {
    use Memo;
    use {FromXdr, ToXdr};

    fn do_it(memo: Memo, expected: &str) {
        let encoded = memo.clone().to_base64().unwrap();
        assert_eq!(encoded, expected);
        let decoded = Memo::from_base64(&encoded).unwrap();
        assert_eq!(decoded, memo);
    }

    #[test]
    fn test_none() {
        let memo = Memo::None;
        do_it(memo, "AAAAAA==");
    }

    #[test]
    fn test_text() {
        let memo = Memo::text("test memo please ignore").unwrap();
        do_it(memo, "AAAAAQAAABd0ZXN0IG1lbW8gcGxlYXNlIGlnbm9yZQA=");
    }

    #[test]
    fn test_id() {
        let memo = Memo::Id(1234);
        do_it(memo, "AAAAAgAAAAAAAATS");
    }

    #[test]
    fn test_hash() {
        let mut buf = [0; 32];
        buf[0] = 1;
        buf[5] = 2;
        buf[13] = 3;
        buf[19] = 4;
        buf[27] = 5;
        let memo = Memo::hash(buf);
        do_it(memo, "AAAAAwEAAAAAAgAAAAAAAAADAAAAAAAEAAAAAAAAAAUAAAAA");
    }

    #[test]
    fn test_return() {
        let mut buf = [0; 32];
        buf[0] = 1;
        buf[5] = 2;
        buf[13] = 3;
        buf[19] = 4;
        buf[27] = 5;
        let memo = Memo::return_(buf);
        do_it(memo, "AAAABAEAAAAAAgAAAAAAAAADAAAAAAAEAAAAAAAAAAUAAAAA");
    }

}
