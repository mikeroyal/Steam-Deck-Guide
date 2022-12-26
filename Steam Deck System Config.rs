// Setting up Steam Deck System Config
pub mod SteamDeck {
   pub fn Hello(name:String) {
      println!("Hello {}",name);
   }
}

fn main(){
   SteamDeck::Hello("Steam Deck users".to_string());
}
