use std::collections::HashMap;

pub enum LinkRefFilter {
    Is(String),
    After(String),
    Before(String),
}

pub enum LinkRef {
    Url(String),
    Filter(Vec<(String, LinkRefFilter)>),
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
    Section { title: String, child: Vec<Node> },
    Tag(String),
    Link { title: String, link_ref: LinkRef },
    XmlTag(String),
}

pub struct Doc {
    properties: HashMap<String, String>,
    children: Vec<Node>
}