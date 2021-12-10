// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {

        if self.health >= 1 {
            None
        } else {
            let mut new_mana = None;
            if self.level >= 10 {
                new_mana = Some(100)
            }

            let new_player = Player {
                health: 100,
                mana: new_mana,
                level: self.level
            };

            Some(new_player) 
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {


        match self.mana {
            None => {
                if self.health < mana_cost {
                    self.health = 0;
                    return 0;
                } else {
                    self.health = self.health - mana_cost;
                    return 0;
                }

            }
            Some(x) => {
                if mana_cost > x {
                    return 0;
                } else {
                    self.mana = Some(x - mana_cost);
                    return mana_cost * 2
                }
            }
        }


    }
}
