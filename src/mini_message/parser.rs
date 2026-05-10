use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'s> {
    #[regex(r"</[a-zA-Z][^>]*>", |lex| lex.slice())]
    CloseTag(&'s str),

    #[regex(r"<[a-zA-Z#!][^>]*/\s*>", |lex| lex.slice())]
    SelfClosing(&'s str),

    #[regex(r"<[a-zA-Z#!][^>]*>", |lex| lex.slice())]
    OpenTag(&'s str),

    #[token(r"\<")]
    EscapedOpen,

    #[regex(r"[^<\\]+", |lex| lex.slice())]
    Text(&'s str),

    #[token("<")]
    BareOpen,
}

#[derive(Debug)]
pub struct TagContent<'s> {
    pub name: &'s str,
    pub args: Vec<&'s str>,
}

impl<'s> TagContent<'s> {
    pub fn parse_open(raw: &'s str) -> Self {
        let inner = raw.trim_matches(|c| c == '<' || c == '>');
        let mut parts = inner.splitn(2, ':');
        let name = parts.next().unwrap_or("");
        let args = parts
            .next()
            .map(|rest| rest.split(':').collect())
            .unwrap_or_default();
        TagContent { name, args }
    }

    pub fn parse_close(raw: &'s str) -> &'s str {
        raw.trim_matches(|c| c == '<' || c == '>' || c == '/')
    }
}

#[derive(Debug)]
pub enum Node<'s> {
    Text(&'s str),
    LiteralOpen,
    Tag {
        name: &'s str,
        args: Vec<&'s str>,
        children: Vec<Node<'s>>,
    },
}

#[derive(Debug)]
pub enum ParseError {
    MismatchedClose { expected: String, got: String },
}

pub struct Parser<'s> {
    src: &'s str,
}

impl<'s> Parser<'s> {
    pub fn new(src: &'s str) -> Self {
        Self { src }
    }

    pub fn parse(&self) -> Result<Vec<Node<'s>>, ParseError> {
        let mut stack: Vec<(&'s str, Vec<&'s str>, Vec<Node<'s>>)> = Vec::new();
        let mut current: Vec<Node<'s>> = Vec::new();

        let mut lexer = Token::lexer(self.src);

        while let Some(token) = lexer.next() {
            let token = match token {
                Ok(t) => t,
                Err(()) => {
                    current.push(Node::Text(lexer.slice()));
                    continue;
                }
            };

            match token {
                Token::Text(s) => current.push(Node::Text(s)),

                Token::EscapedOpen | Token::BareOpen => current.push(Node::LiteralOpen),

                Token::OpenTag(raw) => {
                    let tag = TagContent::parse_open(raw);
                    stack.push((tag.name, tag.args, current));
                    current = Vec::new();
                }

                Token::SelfClosing(raw) => {
                    let inner = raw
                        .trim_matches(|c| c == '<' || c == '>' || c == ' ')
                        .trim_end_matches('/');
                    let mut parts = inner.splitn(2, ':');
                    let name = parts.next().unwrap_or("");
                    let args = parts
                        .next()
                        .map(|rest| rest.split(':').collect())
                        .unwrap_or_default();
                    current.push(Node::Tag {
                        name,
                        args,
                        children: vec![],
                    });
                }

                Token::CloseTag(raw) => {
                    let close_name = TagContent::parse_close(raw);

                    if let Some((open_name, args, mut parent_children)) = stack.pop() {
                        if open_name != close_name {
                            return Err(ParseError::MismatchedClose {
                                expected: open_name.to_string(),
                                got: close_name.to_string(),
                            });
                        }
                        parent_children.push(Node::Tag {
                            name: open_name,
                            args,
                            children: current,
                        });
                        current = parent_children;
                    }
                }
            }
        }

        // Auto-close unclosed tags
        while let Some((name, args, mut parent_children)) = stack.pop() {
            parent_children.push(Node::Tag {
                name,
                args,
                children: current,
            });
            current = parent_children;
        }

        Ok(current)
    }
}
