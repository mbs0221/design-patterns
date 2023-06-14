trait Weapon {
    fn attack(&self);
}

struct Sword;

impl Weapon for Sword {
    fn attack(&self) {
        println!("Sword attack!");
    }
}

struct Bow;

impl Weapon for Bow {
    fn attack(&self) {
        println!("Bow attack!");
    }
}

trait Armor {
    fn defend(&self);
}

struct Shield;

impl Armor for Shield {
    fn defend(&self) {
        println!("Shield defend!");
    }
}

struct ArmorSet;

impl Armor for ArmorSet {
    fn defend(&self) {
        println!("Armor set defend!");
    }
}

trait EquipmentFactory {
    fn create_weapon(&self) -> Box<dyn Weapon>;
    fn create_armor(&self) -> Box<dyn Armor>;
}

struct WarriorEquipmentFactory;

impl EquipmentFactory for WarriorEquipmentFactory {
    fn create_weapon(&self) -> Box<dyn Weapon> {
        Box::new(Sword)
    }

    fn create_armor(&self) -> Box<dyn Armor> {
        Box::new(Shield)
    }
}

struct ArcherEquipmentFactory;

impl EquipmentFactory for ArcherEquipmentFactory {
    fn create_weapon(&self) -> Box<dyn Weapon> {
        Box::new(Bow)
    }

    fn create_armor(&self) -> Box<dyn Armor> {
        Box::new(ArmorSet)
    }
}

struct Character {
    weapon: Box<dyn Weapon>,
    armor: Box<dyn Armor>,
}

impl Character {
    fn new(factory: Box<dyn EquipmentFactory>) -> Character {
        let weapon = factory.create_weapon();
        let armor = factory.create_armor();
        Character { weapon, armor }
    }

    fn attack(&self) {
        self.weapon.attack();
    }

    fn defend(&self) {
        self.armor.defend();
    }
}

fn main() {
    let warrior_factory = Box::new(WarriorEquipmentFactory);
    let warrior = Character::new(warrior_factory);
    warrior.attack();
    warrior.defend();

    let archer_factory = Box::new(ArcherEquipmentFactory);
    let archer = Character::new(archer_factory);
    archer.attack();
    archer.defend();
}
