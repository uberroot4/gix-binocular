#[derive(Debug)]
pub(crate) struct ChangesInfo {
    pub file: String,
    pub insertions: u32,
    pub deletions: u32,
}

#[cfg(test)]
mod tests {
    use crate::objects::changes::ChangesInfo;
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn test_ser_empty() {
        let map = ChangesInfo {
            file: "".to_string(),
            insertions: 0,
            deletions: 0,
        };

        assert_ser_tokens(
            &map,
            &[
                Token::Struct {
                    len: 3,
                    name: "ChangesInfo",
                },
                Token::Str("file"),
                Token::String(""),
                Token::Str("insertions"),
                Token::U32(0),
                Token::Str("deletions"),
                Token::U32(0),
                Token::StructEnd,
            ],
        );
    }
}
