use crate::hero::data::SkillData;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Skill {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub skill_type: String,
    pub level: u32,
}

impl Skill {
    pub fn new(skill_data: &SkillData) -> Self {
        Self {
            id: skill_data.id,
            name: skill_data.name.to_string(),
            description: skill_data.description.to_string(),
            skill_type: skill_data.skill_type.to_string(),
            level: 1,
        }
    }
}
