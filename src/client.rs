#[derive(Clone, Debug)]
pub struct Client {
    base_url: reqwest::Url
}

impl Client {
    pub fn new(base_url: String) -> Option<Self> { 
        let base_url = reqwest::Url::parse(base_url.as_str())
            .ok()?;
        
        return Some(Self { base_url });
    }
    
    pub fn query(self: &Self) -> crate::query::QueryBuilder {
        crate::query::QueryBuilder::new(self.clone().base_url)
    }
}
