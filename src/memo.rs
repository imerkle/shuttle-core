use error::{Error, Result};

const MAX_MEMO_TEXT_LEN: usize = 28;

/// Memo attached to transactions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Memo {
    /// No memo
    None,
    /// Text Memo
    Text(String),
    /// Id Memo
    Id(u64),
    /// Hash Memo
    Hash([u8; 32]),
    /// Return Memo
    Return([u8; 32]),
}

impl Memo {
    /// Create new empty memo.
    pub fn none() -> Memo {
        Memo::None
    }

    /// Create new id memo.
    pub fn id(id: u64) -> Memo {
        Memo::Id(id)
    }

    /// Create new text memo. `text` must be shorter than 28 bytes.
    pub fn text<S: Into<String>>(text: S) -> Result<Memo> {
        let text = text.into();
        if text.len() > MAX_MEMO_TEXT_LEN {
            Err(Error::InvalidMemoText)
        } else {
            Ok(Memo::Text(text))
        }
    }

    /// Create new hash memo.
    pub fn hash(h: [u8; 32]) -> Memo {
        Memo::Hash(h)
    }

    /// Create new return memo.
    pub fn return_(r: [u8; 32]) -> Memo {
        Memo::Return(r)
    }

    /// Return `true` if memo is `None`.
    pub fn is_none(&self) -> bool {
        match *self {
            Memo::None => true,
            _ => false,
        }
    }

    /// Return `true` if memo is `Id`.
    pub fn is_id(&self) -> bool {
        match *self {
            Memo::Id(_) => true,
            _ => false,
        }
    }

    /// Return `true` if memo is `Text`.
    pub fn is_text(&self) -> bool {
        match *self {
            Memo::Text(_) => true,
            _ => false,
        }
    }

    /// Return `true` if memo is `Hash`.
    pub fn is_hash(&self) -> bool {
        match *self {
            Memo::Hash(_) => true,
            _ => false,
        }
    }

    /// Return `true` if memo is `Return`.
    pub fn is_return(&self) -> bool {
        match *self {
            Memo::Return(_) => true,
            _ => false,
        }
    }
}
