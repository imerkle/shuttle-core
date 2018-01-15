use error::{Error, Result};

const MAX_MEMO_TEXT_LEN: usize = 28;

/// Memo attached to transactions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Memo {
    None,
    Id(u64),
    Text(String),
    Hash([u8; 32]),
    Return([u8; 32]),
}

impl Memo {
    pub fn none() -> Memo {
        Memo::None
    }

    pub fn id(id: u64) -> Memo {
        Memo::Id(id)
    }

    pub fn text<S: Into<String>>(t: S) -> Result<Memo> {
        let text = t.into();
        if text.len() > MAX_MEMO_TEXT_LEN {
            Err(Error::TBD)
        } else {
            Ok(Memo::Text(text))
        }
    }

    pub fn hash(h: [u8; 32]) -> Memo {
        Memo::Hash(h)
    }

    pub fn return_(r: [u8; 32]) -> Memo {
        Memo::Return(r)
    }

    pub fn is_none(&self) -> bool {
        match *self {
            Memo::None => true,
            _ => false,
        }
    }

    pub fn is_id(&self) -> bool {
        match *self {
            Memo::Id(_) => true,
            _ => false,
        }
    }

    pub fn is_text(&self) -> bool {
        match *self {
            Memo::Text(_) => true,
            _ => false,
        }
    }

    pub fn is_hash(&self) -> bool {
        match *self {
            Memo::Hash(_) => true,
            _ => false,
        }
    }

    pub fn is_return(&self) -> bool {
        match *self {
            Memo::Return(_) => true,
            _ => false,
        }
    }
}
