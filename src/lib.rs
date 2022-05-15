use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct Skin {
    name: String,
    weapon: String,
    wear: String,
    rarity: String,
    float: Decimal,
    pattern: u16,
    stat_trak: bool,
    
}

blueprint! {
    struct Case {
        system_vault: Vault,
        skin_nft: ResourceAddress,
        collected_xrd: Vault,
        developer_vault: Vault,
        key: ResourceAddress,
        key_price: u8,
        chroma2_skins: HashMap<u8, Vec<String>>,
        chroma2_weapons: HashMap<u8, Vec<String>>,
        skin_float_min_range: HashMap<(String,String), (Decimal,Decimal)>
    }

    impl Case {
        pub fn new() -> (ComponentAddress, Bucket) {

 // Creates developer badge for methods. Necessary to control system_badge
            let mut developer_badge = ResourceBuilder::new_fungible()
                .metadata("name", "developer")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(10000);

            let developer_rule: AccessRule = rule!(require(developer_badge.resource_address()));


            let system_badge = ResourceBuilder::new_fungible()
                .metadata("name", "system")
                .divisibility(DIVISIBILITY_NONE)
                .mintable(developer_rule.clone(), MUTABLE(developer_rule.clone()))
                .initial_supply(1000000);

            let system_rule: AccessRule = rule!(require(system_badge.resource_address()));

            let skin_nft = ResourceBuilder::new_non_fungible()
                .metadata("name", "Weapon Skin NFT")
                .mintable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .burnable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .updateable_non_fungible_data(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .no_initial_supply();

            let key = ResourceBuilder::new_fungible()
                .metadata("name", "Case Key")
                .mintable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .burnable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .no_initial_supply();

            let instantiate = Self {
                system_vault: Vault::with_bucket(system_badge),
                developer_vault: Vault::with_bucket(developer_badge.take(9999)),
                skin_nft,
                collected_xrd: Vault::new(RADIX_TOKEN),
                key,
                key_price: 25,
                chroma2_skins: HashMap::from([
                    (1, vec![String::from("Armor Core"), String::from("Elite Build"),  String::from("Bronze Deco"), 
                        String::from("Man-o'-war"), String::from("Origami"), String::from("Valence")
                        ]),
                    (2, vec![String::from("Pole Position"), String::from("Grand Prix"), 
                        String::from("Heat"), String::from("Worm God")
                        ]),
                    (3, vec![String::from("Djinn"), String::from("Eco"), String::from("Monkey Buisness")
                        ]),
                    (4, vec![String::from("Hyper Beast"), String::from("Neon Rider")
                        ]),
                    // There are vary more knives than 24 skins but Im lazy and want to reuse seed_24
                    // System is the same though no matter how many knives exist
                    (5, vec![String::from("★"), String::from("★"), String::from("★"),
                        String::from("★"), String::from("★"), String::from("★"),
                        String::from("Case Hardened"), String::from("Case Hardened"), String::from("Case Hardened"),
                        String::from("Case Hardened"), String::from("Case Hardened"), String::from("Case Hardened"),
                        String::from("Fade"), String::from("Fade"), String::from("Fade"),
                        String::from("Fade"), String::from("Fade"), String::from("Fade"),
                        String::from("Doppler"), String::from("Doppler"), String::from("Doppler"),
                        String::from("Doppler"), String::from("Doppler"), String::from("Doppler"),
                    ]),
                ]),
                chroma2_weapons: HashMap::from([
                    (1, vec![String::from("MP7"), String::from("AK-47"), String::from("Desert Eagle"),
                        String::from("Negev"), String::from("Sawed-Off"), String::from("P250")
                        ]),
                    (2, vec![String::from("CZ75-Auto"), String::from("UMP-45"), 
                        String::from("MAG-7"), String::from("AWP")
                        ]),
                    (3, vec![String::from("FAMAS"), String::from("Galil AR"), String::from("Five-SeveN")
                        ]),
                    (4, vec![String::from("M4A1-S"), String::from("MAC-10")
                        ]),
                    (5, vec![String::from("Bayonet"), String::from("Karambit"), String::from("Butterfly"),
                        String::from("M9 Bayonet"), String::from("Gut Knife"), String::from("Flip Knife"),
                        String::from("Bayonet"), String::from("Karambit"), String::from("Butterfly"),
                        String::from("M9 Bayonet"), String::from("Gut Knife"), String::from("Flip Knife"),
                        String::from("Bayonet"), String::from("Karambit"), String::from("Butterfly"),
                        String::from("M9 Bayonet"), String::from("Gut Knife"), String::from("Flip Knife"),
                        String::from("Bayonet"), String::from("Karambit"), String::from("Butterfly"),
                        String::from("M9 Bayonet"), String::from("Gut Knife"), String::from("Flip Knife"),
                    ]),
                ]),
                skin_float_min_range: HashMap::from([
                    ((String::from("MP7"),String::from("Armor Core")), (dec!("0"),dec!("1"))),
                    ((String::from("AK-47"),String::from("Elite Build")), (dec!("0"),dec!("1"))),
                    ((String::from("Desert Eagle"),String::from("Bronze Deco")), (dec!("0"),dec!("1"))),
                    ((String::from("Negev"),String::from("Man-o'-war")), (dec!("0"),dec!("1"))),
                    ((String::from("Sawed-Off"),String::from("Origami")), (dec!("0"),dec!("1"))),
                    ((String::from("P250"),String::from("Valence")), (dec!("0"),dec!("1"))),
                    ((String::from("CZ75-Auto"),String::from("Pole Position")), (dec!("0"),dec!("1"))),
                    ((String::from("UMP-45"),String::from("Grand Prix")), (dec!("0"),dec!("1"))),
                    ((String::from("MAG-7"),String::from("Heat")), (dec!("0"),dec!("1"))),
                    ((String::from("AWP"),String::from("Worm God")), (dec!("0"),dec!("1"))),
                    ((String::from("FAMAS"),String::from("Djinn")), (dec!("0"),dec!("1"))),
                    ((String::from("Galil AR"),String::from("Eco")), (dec!("0"),dec!("1"))),
                    ((String::from("Five-SeveN"),String::from("Monkey Buisness")), (dec!("0"),dec!("1"))),
                    ((String::from("M4A1-S"),String::from("Hyper Beast")), (dec!("0"),dec!("1"))),
                    ((String::from("MAC-10"),String::from("Neon Rider")), (dec!("0"),dec!("1"))),
                ]),
            }
            .instantiate()
            .globalize();
            (instantiate, developer_badge)
        }
        pub fn withdraw_xrd(&mut self) -> Bucket {
            let withdraw = self.developer_vault.authorize(||self.collected_xrd.take_all());
            withdraw
        }
        // Buy keys to open cases. I didn't make cases a token cause I'm lazy
        pub fn buy_keys(&mut self, mut payment: Bucket, amount: u32) -> (Bucket, Bucket) {
            let key_bucket: Bucket = self.system_vault.take(1);
            let cost = amount * 25;
            let xrd = payment.take(cost);
            self.collected_xrd.put(xrd);
            let new_keys = self.system_vault.authorize(||
                borrow_resource_manager!(self.key)
                    .mint(amount));
            self.system_vault.put(key_bucket);
            (new_keys, payment)
        }
        // Open a case
        pub fn open_case(&mut self, key: Bucket) -> Bucket {
            assert!(key.amount() == dec!(1));
            let key_bucket: Bucket = self.system_vault.take(1);
            let random = self.random_data();
            let skin = self.chroma2_skins.get(&random.1).unwrap();
            let seed = self.seed24();
            let name = if random.1 == 1 { 
                match seed {
                    0..=3 => &skin[0],
                    4..=7 => &skin[1],
                    8..=11 => &skin[2],
                    12..=15 => &skin[3],
                    16..=19 => &skin[4],
                    20..=23 => &skin[5],
                    _ => &skin[100],
                } 
            }
            else if random.1 == 2 { 
                match seed {
                    0..=5 => &skin[0],
                    6..=11 => &skin[1],
                    12..=17 => &skin[2],
                    18..=23 => &skin[3],
                    _ => &skin[100],
                }  
            }
            else if random.1 == 3 { 
                match seed {
                    0..=7 => &skin[0],
                    8..=15 => &skin[1],
                    16..=23 => &skin[2],
                    _ => &skin[100],
                }  
            }
            else if random.1 == 4 { 
                match seed{
                    0..=11 => &skin[0],
                    12..=23 => &skin[1],
                    _ => &skin[100],
                } 
            }
            else {
                let u: usize = (seed).into();
                &skin[u]
            };
            let gun = self.chroma2_weapons.get(&random.1).unwrap();
            let weapon = if random.1 == 1 { 
                match seed {
                    0..=3 => &gun[0],
                    4..=7 => &gun[1],
                    8..=11 => &gun[2],
                    12..=15 => &gun[3],
                    16..=19 => &gun[4],
                    20..=23 => &gun[5],
                    _ => &gun[100],
                } 
            }
            else if random.1 == 2 { 
                match seed {
                    0..=5 => &gun[0],
                    6..=11 => &gun[1],
                    12..=17 => &gun[2],
                    18..=23 => &gun[3],
                    _ => &gun[100],
                }  
            }
            else if random.1 == 3 { 
                match seed {
                    0..=7 => &gun[0],
                    8..=15 => &gun[1],
                    16..=23 => &gun[2],
                    _ => &gun[100],
                }  
            }
            else if random.1 == 4 { 
                match seed{
                    0..=11 => &gun[0],
                    12..=23 => &gun[1],
                    _ => &gun[100],
                } 
            }
            else {
                let u: usize = (seed).into();
                &gun[u]
            };
            let new_float_data = self.skin_float_min_range.get(&(name.to_string(),weapon.to_string())).unwrap();
            let float_min = new_float_data.0;
            let float_width = new_float_data.1;
            let float = (random.3 * float_width) + float_min;
            let skin_data = Skin {
                rarity: random.2,
                weapon: weapon.to_string(),
                name: name.to_string(),
                wear: random.4,
                float: float,
                pattern: random.0,
                stat_trak: random.5,
            };
            let new_skin = self.system_vault.authorize(||
                borrow_resource_manager!(self.skin_nft)
                    .mint_non_fungible(&NonFungibleId::random(), skin_data));

            self.system_vault.authorize(||
                key.burn());
            self.system_vault.put(key_bucket);
            return new_skin
        }
        pub fn trade_up(&mut self, skins: Bucket) -> Bucket {
            let key_bucket: Bucket = self.system_vault.take(1);
            let seed = self.seed24();
            let random = self.random_data();
            let restricted_skins = self.chroma2_skins.get(&2).unwrap();
            let classified_skins = self.chroma2_skins.get(&3).unwrap();
            let covert_skins = self.chroma2_skins.get(&4).unwrap();
            let restricted_weapons = self.chroma2_weapons.get(&2).unwrap();
            let classified_weapons = self.chroma2_weapons.get(&3).unwrap();
            let covert_weapons = self.chroma2_weapons.get(&4).unwrap();
            assert!(skins.amount() == dec!(10));
            assert!(skins.resource_address() == self.skin_nft);
            let skin1_data: Skin = skins.non_fungibles()[0].data();
            let skin2_data: Skin = skins.non_fungibles()[1].data();
            let skin3_data: Skin = skins.non_fungibles()[2].data();
            let skin4_data: Skin = skins.non_fungibles()[3].data();
            let skin5_data: Skin = skins.non_fungibles()[4].data();
            let skin6_data: Skin = skins.non_fungibles()[5].data();
            let skin7_data: Skin = skins.non_fungibles()[6].data();
            let skin8_data: Skin = skins.non_fungibles()[7].data();
            let skin9_data: Skin = skins.non_fungibles()[8].data();
            let skin10_data: Skin = skins.non_fungibles()[9].data();
            assert!(skin1_data.rarity != String::from("Covert") 
                && skin2_data.rarity == skin1_data.rarity
                && skin3_data.rarity == skin1_data.rarity
                && skin4_data.rarity == skin1_data.rarity
                && skin5_data.rarity == skin1_data.rarity
                && skin6_data.rarity == skin1_data.rarity
                && skin7_data.rarity == skin1_data.rarity
                && skin8_data.rarity == skin1_data.rarity
                && skin9_data.rarity == skin1_data.rarity
                && skin10_data.rarity == skin1_data.rarity
            );

            let stattrak = skin1_data.stat_trak;
            let current_rarity = if  skin1_data.rarity == String::from("Mil-Spec") {
                1
            } else if skin1_data.rarity == String::from("Restricted") {
                2
            } else {
                3
            };
            let name = if current_rarity == 1 { 
                match seed {
                    0..=5 => &restricted_skins[0],
                    6..=11 => &restricted_skins[1],
                    12..=17 => &restricted_skins[2],
                    18..=23 => &restricted_skins[3],
                    _ => &restricted_skins[100],
                }  
            }
            else if current_rarity == 2 { 
                match seed {
                    0..=7 => &classified_skins[0],
                    8..=15 => &classified_skins[1],
                    16..=23 => &classified_skins[2],
                    _ => &classified_skins[100],
                }  
            }
            else { 
                match seed{
                    0..=11 => &covert_skins[0],
                    12..=23 => &covert_skins[1],
                    _ => &covert_skins[100],
                } 
            };
            let weapon = if current_rarity == 1 { 
                match seed {
                    0..=5 => &restricted_weapons[0],
                    6..=11 => &restricted_weapons[1],
                    12..=17 => &restricted_weapons[2],
                    18..=23 => &restricted_weapons[3],
                    _ => &restricted_weapons[100],
                }  
            }
            else if current_rarity == 2 { 
                match seed {
                    0..=7 => &classified_weapons[0],
                    8..=15 => &classified_weapons[1],
                    16..=23 => &classified_weapons[2],
                    _ => &classified_weapons[100],
                }  
            }
            else { 
                match seed{
                    0..=11 => &covert_weapons[0],
                    12..=23 => &covert_weapons[1],
                    _ => &covert_weapons[100],
                } 
            };
            let new_float_data = self.skin_float_min_range.get(&(name.to_string(),weapon.to_string())).unwrap();
            let float_min = new_float_data.0;
            let float_width = new_float_data.1;
            let new_float = (((skin1_data.float + skin2_data.float + skin3_data.float + skin4_data.float + skin5_data.float + 
                skin6_data.float + skin7_data.float + skin8_data.float + skin9_data.float + skin10_data.float)/dec!(10)) * float_width) + float_min;
            let new_wear = if new_float <= dec!(".07") {
                String::from("Factory New")
            }
            else if new_float >= dec!(".07") || new_float == dec!(".07") || new_float <= dec!(".15"){
                String::from("Minimal Wear")
            }
            else if new_float >= dec!(".15") || new_float == dec!(".15") || new_float <= dec!(".38"){
                String::from("Field Tested")
            }
            else if new_float >= dec!(".38") || new_float == dec!(".38") || new_float <= dec!(".45"){
                String::from("Field Tested")
            }
            else {
                String::from("Battle Scared")
            };
            let new_rarity = if skin1_data.rarity == String::from("Mil-Spec") {
                String::from("Restricted")
            } else if skin1_data.rarity == String::from("Restricted") {
                String::from("Classified")
            } else {
                String::from("Covert")
            };
            let skin_data = Skin {
                rarity: new_rarity,
                weapon: weapon.to_string(),
                name: name.to_string(),
                wear: new_wear,
                float: new_float,
                pattern: random.0,
                stat_trak: stattrak,
            };
            let new_skin = self.system_vault.authorize(||
                borrow_resource_manager!(self.skin_nft)
                    .mint_non_fungible(&NonFungibleId::random(), skin_data));

            self.system_vault.authorize(||
                skins.burn());
            self.system_vault.put(key_bucket);
            return new_skin 
        }

        // Randomized data for everything except which skin of each rarity
        pub fn random_data(&self) -> (u16,u8,String,Decimal,String,bool) {
            let mut digits = Vec::new();
            let mut seed = Runtime::generate_uuid();
            while seed > 9 {
                digits.push(seed % 10);
                seed = seed / 10
            }
            digits.push(seed);
            digits.reverse();
            let pattern: u16 = format!("{}{}{}", digits[0], digits[1], digits[2]).parse().unwrap();

            let rarity_number: u32 = format!("{}{}{}{}{}{}{}", digits[3], digits[4], digits[5], 
                digits[6], digits[7], digits[8], digits[9]).parse().unwrap();
            let numba = match rarity_number {
                0..=7992326 => 1,
                7992327..=9590791 => 2,
                9590792..=9910484 => 3,
                9910485..=9974423 => 4,
                9974424..=9999999 => 5,
                _ => 6,
            };
            let rarity = match numba {
                1 => String::from("Mil-Spec"),
                2 => String::from("Restricted"),
                3 => String::from("Classified"),
                4 => String::from("Covert"),
                5 => String::from("Covert"),
                _ => String::from("Error!"),
            };
            let float_format: i128 = format!("{}{}{}{}{}{}{}{}{}{}", digits[10], digits[11], digits[12], 
                digits[13], digits[14], digits[15], digits[16], digits[17], digits[18], digits[19]).parse().unwrap();
            let float_dec: Decimal = float_format.into();
            let big_i: i128 = 9999999999;
            let big_decimal: Decimal = big_i.into();
            let float = float_dec / big_decimal;
            let wear = match float_format {
                0..=0699999999 => String::from("Factory New"),
                0700000000..=1499999999 => String::from("Minimal Wear"),
                1500000000..=3799999999 => String::from("Field Tested"),
                3800000000..=4499999999 => String::from("Well Worn"),
                4500000000..=9999999999 => String::from("Battle Scared"),
                _ => String::from("Error"),
            };
            let stattrak = match digits[22] {
                0..=4 => true,
                _ => false,
            };
            (pattern,numba,rarity,float,wear,stattrak)
        }
        // Randomized number between 0..23 for random skin of a rarity
        pub fn seed24(&self) -> u8 {
            let mut digits = Vec::new();
            let mut seed = Runtime::generate_uuid();
            while seed > 9 {
                digits.push(seed % 10);
                seed = seed / 10
            }
            digits.push(seed);
            digits.reverse();
            let mut counter: usize = 0;
            let mut counter2: usize = 1;
            loop {
                let point = digits[counter];
                let point2 = digits[counter2];
                counter += 2;
                counter2 += 2;
                let random_24: u8 = format!("{}{}", point, point2).parse().unwrap();
                let choice = match random_24 {
                    00..=03 => 1,
                    04..=07 => 2,
                    08..=11 => 3,
                    12..=15 => 4,
                    16..=19 => 5,
                    20..=23 => 6,
                    24..=27 => 7,
                    28..=31 => 8,
                    32..=35 => 9,
                    36..=39 => 10,
                    40..=43 => 11,
                    44..=47 => 12,
                    48..=51 => 13,
                    52..=55 => 14,
                    56..=59 => 15,
                    60..=63 => 16,
                    64..=67 => 17,
                    68..=71 => 18,
                    72..=75 => 19,
                    76..=79 => 20,
                    80..=83 => 21,
                    84..=87 => 22,
                    88..=91 => 23,
                    92..=95 => 0,
                    _ => 24,
                };
                if choice != 24 {
                    return choice
                }
                else {
                    continue
                }
            };
        }
    }
}
