#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub path: String,
}
