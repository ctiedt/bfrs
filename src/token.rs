pub struct Token {
    pub token_type: TokenType,
    pub referencing: Option<usize>,
}

pub enum TokenType {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    AccIn,
    Out,
    LoopBegin,
    LoopEnd,
}

impl Token {
    fn new(token_type: TokenType, referencing: Option<usize>) -> Self {
        Self {
            token_type: token_type,
            referencing: referencing,
        }
    }

    pub fn inc_ptr() -> Self {
        Self::new(TokenType::IncPtr, None)
    }

    pub fn dec_ptr() -> Self {
        Self::new(TokenType::DecPtr, None)
    }

    pub fn inc_val() -> Self {
        Self::new(TokenType::IncVal, None)
    }

    pub fn dec_val() -> Self {
        Self::new(TokenType::DecVal, None)
    }

    pub fn acc_in() -> Self {
        Self::new(TokenType::AccIn, None)
    }

    pub fn out() -> Self {
        Self::new(TokenType::Out, None)
    }

    pub fn loop_begin(referencing: Option<usize>) -> Self {
        Self::new(TokenType::LoopBegin, referencing)
    }

    pub fn loop_end(referencing: usize) -> Self {
        Self::new(TokenType::LoopEnd, Some(referencing))
    }
}
