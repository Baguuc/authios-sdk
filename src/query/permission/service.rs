pub struct ServicePermissionQueryBuilder {
    pub base_url: reqwest::Url,
    pub api_key: String
}

impl ServicePermissionQueryBuilder {
    pub fn create(self, service_id: String) {
        todo!()
    }
    
    pub fn delete(self, service_id: String) {
        todo!()
    }
}
