pub struct RemoteRef(String);
pub struct LocalRef(String);


pub enum Ref {
    Remote(RemoteRef),
    Local(LocalRef),
}

pub enum Citation {
    Remote(RemoteRef),
    Other(String),
}

pub enum Node {
    Section { title: String, children: Vec<Node> },
    Code { lang: String, source: String },
    Citation(Citation),
    Link(Ref),
    Captioned{child: Node, caption:  }
}
