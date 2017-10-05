use std::collections::HashMap;

/// A link in the document. May link to external or internal resources
pub enum LinkRef {
    /// A URL to an external webpage
    Url(String),
    /// A reference to a local document by title
    Local(String),
}

pub enum Prop<T> {
    Value(T),
    Inherit,
}

impl<T> Default for Prop<T> {
    fn default() -> Self {
        Prop::Inherit
    }
}

pub enum Node {
    Code {
        lang: Option<String>,
        source: String,
    },
    Detail {
        why: Option<String>,
        detail: Vec<Node>,
    },
    BlockQuote {
        source: Option<String>,
        quote: String,
    },
    Section { title: String, children: Vec<Node> },
    Tag(String),
    Link { title: String, link_ref: LinkRef },
    XmlTag(String),
}

pub trait Renderer {
    type Output;
    fn render(&self, node: Node) -> Self::Output;
}

pub struct Article {
    author: String,
    date: String,
    title: String,
    content: Vec<Node>
}
