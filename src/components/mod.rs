use std::collections::HashMap;
//this is the big mod file for components that will expand as I add more components

///Component that allows an entity to be rendered, contains the hashmap key needed to retrieve
///the necessary Texture2D from the texture atlas
pub struct Renderable {
    sprite: String,
}
impl Renderable {
    pub fn get_sprite(&self) -> String {
        self.sprite.clone()
    }
}

pub enum WeaponTrait {
    Auto(i32),
}

pub enum WeaponType {
    Ranged(RangedType),
    Melee(MeleeType),
    HeavyWeapon(HeavyWeaponsType),
}

pub enum WeaponSize {
    OneHanded,
    TwoHanded,
}

///Contains all the data necessary for a weapon to be interacted with
pub struct Weapon {
    ///attack type used for determining both rules interactions and what animations will play
    attack_type: WeaponType,
    ///how many hands a weapon uses
    size: WeaponSize,
    ///range is in meters
    range: i32,
    ///damage stored as a string that will can be parsed into a damage roll
    damage: String,
    ///weight in kilograms
    kg: f32,
    ///cost in credits
    cost: i32,
    ///magazine size represented as an i32. note that current ammo will be stored as a separate component
    magazine: i32,
}
pub enum AttributeType {
    Strength,
    Dexterity,
    Endurance,
    Intelligence,
    Education,
    Charm,
}

///component that stores the attributes of an npc inspired by mongoose traveller 2e characteristics
pub struct Attributes {
    ///a character's natural physical strength
    strength: AttributeValue,
    ///a character's agility, reflexes, coordination, and fine motor control
    dexterity: AttributeValue,
    ///a character's physical stamina, determination, and ability to sustain damage
    endurance: AttributeValue,
    ///a character's raw intelligence and quickness of mind - used for new information
    ///and puzzle solving
    intelligence: AttributeValue,
    ///a character's level of lifetime learning and experience especially in academics/intellectual pursuits
    education: AttributeValue,
    ///a character's untrained charisma, social aptitude, and ability to relate to others
    charm: AttributeValue,
}
impl Attributes {
    ///provides an attribute given the attribute type enum to then have further data specified out of it
    pub fn attribute(&self, attribute: AttributeType) -> &AttributeValue {
        match attribute {
            AttributeType::Strength => &self.strength,
            AttributeType::Dexterity => &self.dexterity,
            AttributeType::Endurance => &self.endurance,
            AttributeType::Intelligence => &self.intelligence,
            AttributeType::Education => &self.education,
            AttributeType::Charm => &self.charm,
        }
    }
    ///provides mutable access to a specific attribute
    pub fn mut_attribute(&self, attribute: AttributeType) -> &AttributeValue {
        match attribute {
            AttributeType::Strength => &mut self.strength,
            AttributeType::Dexterity => &mut self.dexterity,
            AttributeType::Endurance => &mut self.endurance,
            AttributeType::Intelligence => &mut self.intelligence,
            AttributeType::Education => &mut self.education,
            AttributeType::Charm => &mut self.charm,
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
///Holds all necessary data about the character's attributes
pub struct AttributeValue {
    current: i32,
    max: i32,
}
impl AttributeValue {
    ///gets the current value of the given attribute. Used for almost all circumstances
    ///the game checks the attribute's value
    pub fn current(&self) -> i32 {
        self.current
    }
    ///gets the maximum value of the given attribute
    pub fn max(&self) -> i32 {
        self.max
    }
    ///Get the current bonus of the attribute used for almost all skill checks
    pub fn bonus(&self) -> i32 {
        match self.current {
            ..=0 => -3,
            1..=2 => -2,
            3..=5 => -1,
            9..=11 => 1,
            12..=14 => 2,
            15.. => 3,
        }
    }
    ///Heal the attribute by a given delta, up to the maximum value of the attribute
    pub fn heal(&mut self, delta: i32) {
        if delta.is_positive() {
            if (self.current + delta) <= self.max {
                self.current += delta;
            } else {
                self.current = self.max;
            }
        }
    }
    ///Damage the attribute by a given delta
    pub fn damage(&mut self, delta: i32) {
        if delta.is_positive() {
            self.current -= delta;
        }
    }
}

///Component that holds all of the known skills a character has
pub struct Skills {
    // known_skills: HashMap<SkillType, i32>,
    known_skills: HashMap<SkillType, i32>,
}
impl Skills {
    ///Used to get the dice modifier of the skill level a character has
    pub fn get_dm(&self, skill: SkillType) -> i32 {
        //if a character has training in the skill return their skill level
        //if a character has training in the general skill but not the specific specialty return 0
        //if a character has no training at all return -3
        match skill {
            SkillType::Melee(subskill) => {
                //first check for training in a specific subskill
                if self.known_skills.contains_key(&skill(subskill)) {
                    *self.known_skills.get(&skill(subskill)).unwrap()
                //then check to see if they have training in the general skill
                } else if self.known_skills.contains_key(&skill(None)) {
                    0
                //if not return -3 like other skills
                } else {
                    -3
                }
            }
            SkillType::Ranged(subskill) => {
                //first check for training in a specific subskill
                if self.known_skills.contains_key(&skill(subskill)) {
                    *self.known_skills.get(&skill(subskill)).unwrap()
                //then check to see if they have training in the general skill
                } else if self.known_skills.contains_key(&skill(None)) {
                    0
                //if not return -3 like other skills
                } else {
                    -3
                }
            }
            SkillType::Explosives => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::HeavyWeapons(subskill) => {
                //first check for training in a specific subskill
                if self.known_skills.contains_key(&skill(subskill)) {
                    *self.known_skills.get(&skill(subskill)).unwrap()
                //then check to see if they have training in the general skill
                } else if self.known_skills.contains_key(&skill(None)) {
                    0
                //if not return -3 like other skills
                } else {
                    -3
                }
            }
            SkillType::Broker => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Persuade => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Streetwise => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Deception => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Leadership => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Diplomat => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Electronics => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Investigate => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Mechanic => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Medic => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Admin => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Advocate => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Science(subskill) => {
                //first check for training in a specific subskill
                if self.known_skills.contains_key(&skill(subskill)) {
                    *self.known_skills.get(&skill(subskill)).unwrap()
                //then check to see if they have training in the general skill
                } else if self.known_skills.contains_key(&skill(None)) {
                    0
                //if not return -3 like other skills
                } else {
                    -3
                }
            }
            SkillType::Language(subskill) => {
                //first check for training in a specific subskill
                if self.known_skills.contains_key(&skill(subskill)) {
                    *self.known_skills.get(&skill(subskill)).unwrap()
                //then check to see if they have training in the general skill
                } else if self.known_skills.contains_key(&skill(None)) {
                    0
                //if not return -3 like other skills
                } else {
                    -3
                }
            }
            SkillType::Athletics(subskill) => {
                //first check for training in a specific subskill
                if self.known_skills.contains_key(&skill(subskill)) {
                    *self.known_skills.get(&skill(subskill)).unwrap()
                //then check to see if they have training in the general skill
                } else if self.known_skills.contains_key(&skill(None)) {
                    0
                //if not return -3 like other skills
                } else {
                    -3
                }
            }
            SkillType::Stealth => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Survival => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Recon => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Animals(subskill) => {
                //first check for training in a specific subskill
                if self.known_skills.contains_key(&skill(subskill)) {
                    *self.known_skills.get(&skill(subskill)).unwrap()
                //then check to see if they have training in the general skill
                } else if self.known_skills.contains_key(&skill(None)) {
                    0
                //if not return -3 like other skills
                } else {
                    -3
                }
            }
            SkillType::Art(subskill) => {
                //first check for training in a specific subskill
                if self.known_skills.contains_key(&skill(subskill)) {
                    *self.known_skills.get(&skill(subskill)).unwrap()
                //then check to see if they have training in the general skill
                } else if self.known_skills.contains_key(&skill(None)) {
                    0
                //if not return -3 like other skills
                } else {
                    -3
                }
            }
            SkillType::Carouse => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Drive(subskill) => {
                //first check for training in a specific subskill
                if self.known_skills.contains_key(&skill(subskill)) {
                    *self.known_skills.get(&skill(subskill)).unwrap()
                //then check to see if they have training in the general skill
                } else if self.known_skills.contains_key(&skill(None)) {
                    0
                //if not return -3 like other skills
                } else {
                    -3
                }
            }
            SkillType::Gambler => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::Navigation => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
            SkillType::VaccSuit => {
                if self.known_skills.contains_key(&skill) {
                    *self.known_skills.get(&skill).unwrap()
                } else {
                    -3
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum SkillType {
    /*Combat Skills*/
    //primary combat skills
    Melee(Option<MeleeType>),
    Ranged(Option<RangedType>),
    //secondary combat skills
    Explosives,
    HeavyWeapons(Option<HeavyWeaponsType>),
    /*Social Skills*/
    //primary social skills
    Broker,
    Persuade,
    Streetwise,
    //secondary social skills
    Deception,
    Leadership,
    Diplomat,
    /*Knowledge Skills*/
    //primary knowledge skills
    Electronics,
    Investigate,
    Mechanic,
    Medic,
    //secondary knowledge skills
    Admin,
    Advocate,
    Science(Option<ScienceType>),
    Language(Option<LanguageType>),
    /*Misc Skills*/
    //primary misc skills
    Athletics(Option<AthleticsType>),
    Stealth,
    Survival,
    Recon,
    //secondary misc skills
    Animals(Option<AnimalsType>),
    Art(Option<ArtType>), //this one is the most likely to be useless
    Carouse,
    Drive(Option<DriveType>),
    Gambler,
    Navigation,
    VaccSuit,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum AthleticsType {
    Dexterity,
    Endurance,
    Strength,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum MeleeType {
    Unarmed,
    Blade,
    Bludgeon,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum RangedType {
    OneHanded,
    TwoHanded,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum HeavyWeaponsType {
    Artillery,
    Portable,
    Vehicle,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum ScienceType {
    Biology,
    Chemistry,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum LanguageType {
    Basic,
    BinaricCant,
    DeepAsh,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum AnimalsType {
    Handling,
    Veterinary,
    Training,
}

//this is the most likely skill to be cut
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum ArtType {
    Performer,
    Instrument,
    VisualMedia,
    Write,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum DriveType {
    Track,
    Walker,
    Wheel,
}
