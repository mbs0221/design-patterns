trait Character {
    fn create() -> Self;
    fn display(&self);
}

struct Warrior;
struct Mage;
struct Thief;

impl Character for Warrior {
    fn create() -> Self {
        Warrior
    }

    fn display(&self) {
        println!("Warrior character created.");
    }
}

impl Character for Mage {
    fn create() -> Self {
        Mage
    }

    fn display(&self) {
        println!("Mage character created.");
    }
}

impl Character for Thief {
    fn create() -> Self {
        Thief
    }

    fn display(&self) {
        println!("Thief character created.");
    }
}

struct CharacterFactory;

impl CharacterFactory {
    fn create_character(character_type: &str) -> Box<dyn Character> {
        match character_type {
            "Warrior" => Box::new(Warrior::create()),
            "Mage" => Box::new(Mage::create()),
            "Thief" => Box::new(Thief::create()),
            _ => panic!("Invalid character type."),
        }
    }
}

fn main() {
    let warrior = CharacterFactory::create_character("Warrior");
    warrior.display();

    let mage = CharacterFactory::create_character("Mage");
    mage.display();

    let thief = CharacterFactory::create_character("Thief");
    thief.display();
}
