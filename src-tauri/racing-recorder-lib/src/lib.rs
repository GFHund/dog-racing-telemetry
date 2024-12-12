#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod common;
mod raceroom;
mod acc;
use common::RecorderTrait::Recorder;
use raceroom::RaceRoom;
use acc::Acc;


pub enum Game{
    RaceRoom,
    AssettoCorsaCompetizione
}

pub struct RecorderBuilder{
    game: Game,
    saveToDataBase: bool
}

impl RecorderBuilder{
    pub fn new()->RecorderBuilder{
        return RecorderBuilder{game: Game::AssettoCorsaCompetizione,saveToDataBase: false};
    }
    pub fn setGame(&mut self,game:Game)-> &mut RecorderBuilder{
        self.game = game;
        return self;
    }
    pub fn setSaveToDatabase(&mut self,saveToDatabase: bool) -> &mut RecorderBuilder{
        self.saveToDataBase = saveToDatabase;
        return self;
    }
    
    pub fn build(&self)->Box<dyn Recorder + Send>{
        match self.game{
            Game::RaceRoom => Box::new(RaceRoom::new()),
            Game::AssettoCorsaCompetizione => Box::new(Acc::new())
        }
        /*
        if self.saveToDataBase{
            match self.game{
                Game::RaceRoom => (),
                Game::AssettoCorsaCompetizione => ()
            }
        } else {
            
        }
         */
    }
     
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe{
            let result = test(2, 2);
            assert_eq!(result, 4);
        }
        
    }
}
 */