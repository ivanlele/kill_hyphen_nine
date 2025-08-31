use super::computer::Computer;

struct Skill {
    name: String,
    level: u64,
    skill_type: SkillType,
    activate: fn(&mut Computer)
}

pub enum SkillType {
    Locator,
    Terminator,
    Protector,
}
