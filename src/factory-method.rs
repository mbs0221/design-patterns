trait Character {
    fn display(&self);
}

struct Warrior;
struct Mage;
struct Thief;

impl Character for Warrior {
    fn display(&self) {
        println!("Warrior character created.");
    }
}

impl Character for Mage {
    fn display(&self) {
        println!("Mage character created.");
    }
}

impl Character for Thief {
    fn display(&self) {
        println!("Thief character created.");
    }
}

trait CharacterFactory {
    fn create_character(&self) -> Box<dyn Character>;
}

struct WarriorFactory;

impl CharacterFactory for WarriorFactory {
    fn create_character(&self) -> Box<dyn Character> {
        Box::new(Warrior)
    }
}

struct MageFactory;

impl CharacterFactory for MageFactory {
    fn create_character(&self) -> Box<dyn Character> {
        Box::new(Mage)
    }
}

struct ThiefFactory;

impl CharacterFactory for ThiefFactory {
    fn create_character(&self) -> Box<dyn Character> {
        Box::new(Thief)
    }
}

fn main() {
    let warrior_factory = WarriorFactory;
    let warrior = warrior_factory.create_character();
    warrior.display();

    let mage_factory = MageFactory;
    let mage = mage_factory.create_character();
    mage.display();

    let thief_factory = ThiefFactory;
    let thief = thief_factory.create_character();
    thief.display();
}
