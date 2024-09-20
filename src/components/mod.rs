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
pub enum AttributeType {
    Strength,
    Dexterity,
    Endurance,
    Intelligence,
    Education,
    Charm,
}

pub struct AttributeValue {
    current: i32,
    max: i32,
}
impl AttributeValue {
    pub fn current(&self) -> i32 {
        self.current
    }
    pub fn max(&self) -> i32 {
        self.max
    }
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
    ///gets the raw attribute score given the attribute type enum
    pub fn get_attribute(&self, attribute: AttributeType) -> i32 {
        match attribute {
            AttributeType::Strength => self.strength.current,
            AttributeType::Dexterity => self.dexterity.current,
            AttributeType::Endurance => self.endurance.current,
            AttributeType::Intelligence => self.intelligence.current,
            AttributeType::Education => self.education.current,
            AttributeType::Charm => self.charm.current,
        }
    }
    ///gets the attribute bonus of the given attribute type, used heavily for skill checks.
    pub fn get_attribute_bonus(&self, attribute: AttributeType) -> i32 {
        let score = self.get_attribute(attribute);
        if score <= 0 {
            -3
        } else if score >= 1 && score <= 2 {
            -2
        } else if score >= 3 && score <= 5 {
            -1
        } else if score >= 6 && score <= 8 {
            0
        } else if score >= 9 && score <= 11 {
            1
        } else if score >= 12 && score <= 14 {
            2
        } else if score >= 15 {
            3
        } else {
            0
        }
    }
}
