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
