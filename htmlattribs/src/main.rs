use std::{collections::HashSet, env, io};

use html5tokenizer::{BufReadReader, Tokenizer};

fn main() -> io::Result<()> {
    let filter_attrs: HashSet<_> = env::args()
        .skip(1)
        .map(|s| s.to_ascii_lowercase())
        .collect();
    let tokenizer = Tokenizer::new(BufReadReader::new(io::stdin().lock()));
    for token in tokenizer {
        let token = token?;
        match token {
            html5tokenizer::Token::StartTag(tag) => {
                let attr_values = tag.attributes.iter().filter_map(|(name, attr)| {
                    (attr.value != "" && (filter_attrs.is_empty() || filter_attrs.contains(name)))
                        .then_some(&attr.value)
                });
                for value in attr_values {
                    println!("{value}");
                }
            }
            html5tokenizer::Token::Error { .. } => {
                // skip
            }
            _ => (),
        }
    }
    Ok(())
}
