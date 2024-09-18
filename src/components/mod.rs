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

pub enum AttackType {
    Ranged,
    Melee,
}

pub enum WeaponSize {
    OneHanded,
    TwoHanded,
}

pub struct Weapon {
    ///attack type used for determining both rules interactions and what animations will play
    attack_type: AttackType,
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
pub enum Attribute {
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
    strength: i32,
    ///a character's agility, reflexes, coordination, and fine motor control
    dexterity: i32,
    ///a character's physical stamina, determination, and ability to sustain damage
    endurance: i32,
    ///a character's raw intelligence and quickness of mind - used for new information
    ///and puzzle solving
    intelligence: i32,
    ///a character's level of lifetime learning and experience especially in academics/intellectual pursuits
    education: i32,
    ///a character's untrained charisma, social aptitude, and ability to relate to others
    charm: i32,
}
impl Attributes {
    ///gets the raw attribute score given the attribute type enum
    pub fn get_attribute(&self, attribute: Attribute) -> i32 {
        match attribute {
            Attribute::Strength => self.strength,
            Attribute::Dexterity => self.dexterity,
            Attribute::Endurance => self.endurance,
            Attribute::Intelligence => self.intelligence,
            Attribute::Education => self.education,
            Attribute::Charm => self.charm,
        }
    }
    ///gets the attribute bonus of the given attribute type, used heavily for skill checks.
    pub fn get_attribute_bonus(&self, attribute: Attribute) -> i32 {}
}
