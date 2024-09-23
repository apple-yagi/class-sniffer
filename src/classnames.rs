use cssparser::{Parser, ParserInput, Token};

// クラス名を抽出する関数
pub fn extract(css_content: &str) -> Vec<String> {
    let mut classnames = Vec::new();

    // パーサーの入力を作成
    let mut input = ParserInput::new(css_content);
    let mut parser = Parser::new(&mut input);

    // トークンを解析してクラス名を抽出
    while let Ok(token) = parser.next_including_whitespace_and_comments() {
        match token {
            Token::Delim('.') => {
                // '.' の後に続くクラス名を取得
                if let Ok(Token::Ident(ident)) = parser.next() {
                    classnames.push(ident.as_ref().to_string());
                }
            }
            _ => {}
        }
    }

    classnames
}

#[test]
fn test_extract() {
    let css_content = r#"
        .foo {
            color: red;
        }
        .bar {
            color: blue;
        }
    "#;
    let classnames = extract(css_content);
    assert_eq!(classnames, vec!["foo", "bar"]);
}
