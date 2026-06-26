#[derive(serde::Deserialize, serde::Serialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub level: u32,
    pub skill_type: String,
}

impl Skill {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            level: 0,
            skill_type: String::new()
        }
    }
}
