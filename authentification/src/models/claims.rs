use std::collections::BTreeMap;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Claims<'a> {
    pub id: i32,
    pub email: &'a str,
    pub name: &'a str,
    pub image: &'a str,
}
impl Claims<'_> {
    pub fn to_jwt_claims(&self) -> BTreeMap<String, serde_json::Value> {
        let mut result = BTreeMap::new();
        result.insert("id".to_string(), serde_json::Value::from(self.id));
        result.insert(
            "email".to_string(),
            serde_json::Value::from(self.email.to_string()),
        );
        result.insert(
            "name".to_string(),
            serde_json::Value::from(self.name.to_string()),
        );
        result.insert(
            "image".to_string(),
            serde_json::Value::from(self.image.to_string()),
        );
        result
    }
}
