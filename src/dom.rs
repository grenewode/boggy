use std::io::Write;

/// A link in the document. May link to external or internal resources
pub enum LinkRef {
    /// A URL to an external webpage
    Url(String),
    /// A reference to a local document by title
    Local(String),
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

pub struct Article {
    author: String,
    date: String,
    title: String,
    content: Vec<Node>,
}

pub trait Renderer {
    fn render(&self, article: Article, write: &mut Write);
}
