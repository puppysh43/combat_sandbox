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

///This enum will be used in this sandbox simply for determining which team someone is on but in the future
///will be used for determining who in combat is controlled by the player and who by AI
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ControlType {
    PC,
    NPC,
}

///A collection of misc. effects that can be given to weapons to represent various features
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum WeaponTrait {
    ///Armor Piercing and will ignore the amount of protection equal to the contained integer value
    AP(i32),
    ///to be implemented later
    Artillery,
    ///Capable of automatic fire giving the weapon access to 3 different fire modes: single, burst, and full auto
    ///single attacks are normal, burst attacks get the contained int value added to the damage and use the int value of rounds
    ///full auto attacks can make a number of attacks equal to the contained int value against any targets w/in 4 tiles of each other
    ///full auto uses a number of rounds equal to 3x the contained int value
    Auto(i32),
    ///Has an explosive or AoE component. On a successful attack damage is rolled against every target w/in a circle
    ///w/ the radius of the contained int value (in meters or tiles undecided). Targeted enemies can't make a a dodge
    ///reaction but can dive for cover. cover is calculated from the center of the blast not the tile of the attacker
    ///NEED TO DECIDE HOW ATTACK ROLLS FOR THIS ARE IMPLEMENTED will probably be a late addition
    Blast(i32),
    ///Has powerful recoil or is just extremely heavy and requires STR 9 or higher to use w/out penalty
    ///if the user doesn't have enough STR the attack rolls will have a negative DM equal to the difference
    ///between their STR DM and +1
    ///Alternatively just check for the DM and if it's less than 1 get the difference
    Bulky,
    ///Has some form of magnified optic, allowing the weapon to ignore standard limits on range that make
    ///all attacks made at over 100 meters (67 tiles) be counted as at Extreme Range as long as the user
    ///aims before shooting
    Scope,
    ///Designed to deal non-lethal damage, incapitating rather than killing. Damage is only deducted from END,
    ///reduced by protection as normal. If the target's END is reduced to 0 by a stun weapon the target will be
    ///unable to do any actions for a num of rounds equal to how much the dmg exceeded their END(or how negative their DM is)
    ///Damage dealt by stun weapons is completely healed by one hour of rest.
    Stun,
    ///Extremely heavy or very intense recoil. It requires STR 12/an STR DM of +2 to use w/out penalty
    ///if not all attack rolls have a negative DM to the attack rolls equal to the difference between their STR DM and +2
    VeryBulky,
    ///Whether through poor quality or inherent flaws the weapon can be as lethal to the user as the target
    ///If an attack roll is made w/ the weapon w/ Effect -5 or worse it explodes inflicting the damage roll on the user
    ///and not the target, and then the weapon is broken beyond use.
    Dangerous,
    ///Sets the target on fire, causing dmg every round after the initial attack. A target can only be on fire from one weapon
    ///at once using the highest damage fire weapon. On its own a fire will extinguish itself on a 2D roll of 8+, rolled at the
    ///start of every round. A character may use 2 Action Points to extinguish the fire requiring an avg (8+) DEX check.
    ///the character gains DM+2 if using firefighting equipment. WILL BE IMPLEMENTED MUCH LATER IF AT ALL
    Fire,
    ///Is designed to be disposable. After being used once the weapon will be rendered inoperable
    OneUse,
    ///Either through compensating for weapon noise or never making it at all the weapon is functionally silent.
    ///Any attempts to detect the source of this sound suffer DM-6
    Silent,
    ///Is particularly heavy and has immense momentum when swung. A character attacked by a weapon w/ the Smasher trait
    ///may not attempt to parry it when reacting to the attack.
    Smasher,
    ///Extremely unstable. If an attack roll is made with Effect -3 or worse it explodes and the damage roll is inflicted
    ///upon the person firing it and the weapon is rendered inoperable
    VeryDangerous,
    ///This weapon isn't designed for harm but is instead repurposed in the heat of battle. It otherwise resembles an existing weapon
    ///but has DM-1 to attack rolls and -1 to each damage dice. After any attack roll w/ an Effect of -3 or less it breaks permanently.
    Improvised,
    ///Has been finely tuned for greater precision. It gets DM+1 to attack rolls
    Accurate,
    ///Has been tweaked for greater than usual range. The range of this weapon is increased by 50% [unsure if I'll implement this one]
    LongRange,
}

///Enum for identifying what skill is used for the attack roll of a weapon
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WeaponType {
    MeleeUnarmed,
    MeleeBlades,
    MeleeBludgeoning,
    RangedOneHanded,
    RangedTwoHanded,
    HeavyWeaponsArtillery,
    HeavyWeaponsPortable,
    HeavyWeaponsVehicle,
    Thrown, //for thrown weapons Athletics(dexterity) is used
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
    //magazine_cost: i32,
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
    pub fn mut_attribute(&mut self, attribute: AttributeType) -> &AttributeValue {
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
    known_skills: HashMap<SkillType, i32>,
}
impl Skills {
    fn get_base_skill(skill: SkillType) -> Option<SkillType> {
        match skill {
            SkillType::MeleeUnarmed => Some(SkillType::Melee),
            SkillType::MeleeBlades => Some(SkillType::Melee),
            SkillType::MeleeBludgeoning => Some(SkillType::Melee),
            SkillType::RangedOneHanded => Some(SkillType::Ranged),
            SkillType::RangedTwoHanded => Some(SkillType::Ranged),
            SkillType::HeavyWeaponsArtillery => Some(SkillType::HeavyWeapons),
            SkillType::HeavyWeaponsPortable => Some(SkillType::HeavyWeapons),
            SkillType::HeavyWeaponsVehicle => Some(SkillType::HeavyWeapons),
            SkillType::AthleticsDexterity => Some(SkillType::Athletics),
            SkillType::AthleticsEndurance => Some(SkillType::Athletics),
            SkillType::AthleticsStrength => Some(SkillType::Athletics),
            SkillType::DriveWheels => Some(SkillType::Drive),
            SkillType::DriveWalker => Some(SkillType::Drive),
            SkillType::DriveTracked => Some(SkillType::Drive),
            _ => None,
        }
    }
    ///Used to get the dice modifier of the skill level a character has
    pub fn get_dm(&self, skill: SkillType) -> i32 {
        //First check if the player actually has the specific skill and if there are any associated base skills
        match (self.known_skills.get(&skill), Skills::get_base_skill(skill)) {
            //if the player has the specific skill being asked for then just return their skill level!
            (Some(skill_val), _) => *skill_val,
            //if they don't have the specific skill being asked for but the skill is one that's part of a general skill w/ specialties
            (None, Some(base_skill)) => {
                //then check if they have training in the base skill associated w/ the one yr checking for
                if self.known_skills.get(&base_skill).is_some() {
                    //and return 0 to represent a base level of competence
                    0
                } else {
                    //if it's a specialty in a general skill they have no training in then they have a DM of -3
                    -3
                }
            }
            //if they don't have the skill and it's not a skill w/ specialties just return the -3 DM
            (None, None) => -3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum SkillType {
    /*Combat Skills*/
    //primary combat skills
    Melee,
    MeleeUnarmed,
    MeleeBlades,
    MeleeBludgeoning,
    Ranged,
    RangedOneHanded,
    RangedTwoHanded,
    //secondary combat skills
    Explosives,
    HeavyWeapons,
    HeavyWeaponsArtillery,
    HeavyWeaponsPortable,
    HeavyWeaponsVehicle,
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
    Science,
    //possibly science subtypes
    LanguageBasic,
    LanguageBinaricCant,
    LanguageOuterAsh,
    /*Misc Skills*/
    //primary misc skills
    Athletics,
    AthleticsDexterity,
    AthleticsEndurance,
    AthleticsStrength,
    Stealth,
    Survival,
    Recon,
    //secondary misc skills
    AnimalHandling,
    Carouse,
    Drive,
    DriveWheels,
    DriveWalker,
    DriveTracked,
    Gambler,
    Navigation,
    VaccSuit,
}
