use std::io::Write;

pub struct Code {
    lang: Option<String>,
    source: String,
}

impl<S: Into<String>> From<S> for Code {
    fn from(source: S) -> Self {
        Code::new(source)
    }
}

impl Code {
    pub fn new<S: Into<String>>(source: S) -> Self {
        Code {
            lang: None,
            source: source.into(),
        }
    }

    pub fn lang<L: Into<String>>(self, lang: L) -> Self {
        let mut s = self;
        s.lang = Some(lang.into());
        s
    }
}

pub struct Detail {
    detail: Vec<Node>,
}

pub struct BlockQuote {
    source: Option<String>,
    quote: String,
}

pub struct Section {
    title: String,
    children: Vec<Node>,
}

pub struct Link {
    title: String,
    link_ref: LinkRef,
}

pub enum LocalRef {
    Title(String),
    Tag(String),
}

/// A link in the document. May link to external or internal resources
pub enum LinkRef {
    /// A URL to an external webpage
    Url(String),
    /// A reference to a local document by title
    Local(LocalRef),
}

pub struct Definition {
    title: Option<String>,
    content: Box<Node>,
}

pub enum Node {
    Code(Code),
    Detail(Detail),
    BlockQuote(BlockQuote),
    Section(Section),
    Link(Link),
    OrderedList { items: Vec<Node> },
    UnorderedList { items: Vec<Node> },
    DefinitionList { items: Vec<Definition> },
    XmlTag(String),
}

pub struct Article {
    title: String,
    author: String,
    date: String,
    tags: Vec<String>,
    content: Vec<Node>,
}

pub trait Renderer<T> {
    fn render(&self, target: T, write: &mut Write);
}
