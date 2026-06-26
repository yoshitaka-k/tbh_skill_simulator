#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub level: u32,
    pub skill_type: String,
}

impl Skill {
    pub fn new(name: String, description: String, level: u32, skill_type: String) -> Self {
        Self { name, description, level, skill_type }
    }
}
