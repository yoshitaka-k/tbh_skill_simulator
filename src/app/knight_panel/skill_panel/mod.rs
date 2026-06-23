/// スキル画像を表す。
pub(crate) struct SkillImage {
    pub image: egui::ImageSource<'static>,
    pub name: &'static str,
    pub max_level: u8,
}

impl SkillImage {
    /// 新しいスキル画像を作成する。
    pub const fn new(
        image: egui::ImageSource<'static>,
        name: &'static str,
        max_level: u8,
    ) -> Self {
        Self { image, name, max_level }
    }
}

pub(crate) const SKILL_IMAGES: &[&[SkillImage]] = &[
    &[
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_attack_damage.png"),
            "Passive Attack Damage",
            3,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_max_hp.png"),
            "Passive Max HP",
            8,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/skill_piercing_thrust.png"),
            "Skill Piercing Thrust",
            5,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/skill_shield_charge.png"),
            "Skill Shield Charge",
            5,
        ),
    ],
    &[
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_armor.png"),
            "Passive Armor",
            8,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_hp_regen_per_sec.png"),
            "Passive HP Regen Per Sec",
            5,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/skill_retribution_strike.png"),
            "Skill Retribution Strike",
            5,
        ),
    ],
    &[
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_add_hp_per_kill.png"),
            "Passive Add HP Per Kill",
            8,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_block_chance.png"),
            "Passive Block Chance",
            8,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/skill_aegis_field.png"),
            "Skill Aegis Field",
            5,
        ),
    ],
    &[
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_physical_damage_percent.png"),
            "Passive Physical Damage Percent",
            8,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_max_hp.png"),
            "Passive Max HP",
            8,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/skill_sacred_blade.png"),
            "Skill Sacred Blade",
            5,
        ),
    ],
    &[
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_cooldown_reduction.png"),
            "Passive Cooldown Reduction",
            8,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_hp_regen_per_sec.png"),
            "Passive HP Regen Per Sec",
            5,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/skill_unyielding_will.png"),
            "Skill Unyielding Will",
            5,
        ),
    ],
    &[
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_add_hp_per_kill.png"),
            "Passive Add HP Per Kill",
            8,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_max_hp.png"),
            "Passive Max HP",
            8,
        ),
    ],
    &[
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_attack_speed.png"),
            "Passive Attack Speed",
            8,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_all_elemental_resistance.png"),
            "Passive All Elemental Resistance",
            8,
        ),
    ],
    &[
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_damage_reduction.png"),
            "Passive Damage Reduction",
            8,
        ),
        SkillImage::new(
            egui::include_image!("../../../../assets/images/knight/passive_attack_damage.png"),
            "Passive Attack Damage",
            3,
        ),
    ],
];

/// スキル一覧を横並びで表示する。
pub(crate) fn skill_row(ui: &mut egui::Ui, skills: &[SkillImage], current_level: u8) {
    ui.horizontal(|ui| {
        for (i, skill) in skills.iter().enumerate() {
            if i > 0 {
                ui.separator();
            }

            ui.vertical(|ui| {
                if ui
                    .add(
                        egui::Button::image(
                            egui::Image::new(skill.image.clone()).fit_to_original_size(1.0),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    println!("{}", skill.name);
                }
                ui.label(format!("{}/{}", current_level, skill.max_level));
            });
        }
    });
}
