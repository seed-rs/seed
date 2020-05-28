use std::collections::HashMap;

#[derive(Debug)]
pub struct Resource(HashMap<String, String>);

impl Default for Resource {
    fn default() -> Self {
        let mut m = HashMap::new();
        m.insert(
            String::from("en-US"),
            include_str!("resources/english.ftl").to_string(),
        );
        Self(m)
    }
}

impl Resource {
    pub fn new() -> Self {
        let mut r = Self::default();
        r.insert(
            "de-DE".to_string(),
            include_str!("resources/german.ftl").to_string(),
        );
        r
    }
    pub fn insert(&mut self, k: String, v: String) -> &Self {
        self.0.insert(k, v);
        self
    }
    pub fn get(&self, k: &str) -> Option<&String> {
        self.0.get(k)
    }
}
