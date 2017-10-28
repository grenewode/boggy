pub struct RemoteRef(String);
pub struct LocalRef(String);


pub enum Ref {
    Remote(RemoteRef),
    Local(LocalRef),
}

pub struct ListItem {
    title: String,
    content: Option<Node>,
}

pub enum Node {
    Code { lang: String, source: String },
    Link(Ref),
    Image {
        local_path: String,
        alt: Option<String>,
    },
    Items(Vec<ListItem>),
    OrderedItems(Vec<ListItem>),
    Definitions(Vec<ListItem>),
    
}
