use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand::Rng;
use std::io;

pub mod cltrctoexit;

#[derive(Clone)]
pub struct Player{
    name: String,
    health: f32,
    attack: i32,
    defense: i32,
}


#[allow(unused_variables)] 
fn main(){
    let  mut characters_list: HashMap<i32, Player> = HashMap::new();
    
    let mut rng = rand::thread_rng();


    characters_list.insert(1, Player { name: String::from("Rockky"), health: 100.0, attack: rng.gen_range(10..20), defense: rng.gen_range(5..20) });
    characters_list.insert(2, Player { name: String::from("ArmStrong"), health: 100.0, attack: rng.gen_range(10..20), defense: rng.gen_range(5..20) });
    characters_list.insert(3, Player { name: String::from("MayNeel"), health: 100.0, attack: rng.gen_range(10..20), defense: rng.gen_range(5..20) });
    characters_list.insert(4, Player { name: String::from("Charsiboy"), health: 100.0, attack: rng.gen_range(10..20), defense: rng.gen_range(5..20) });
    characters_list.insert(5, Player { name: String::from("Daarue"), health: 100.0, attack: rng.gen_range(10..20), defense: rng.gen_range(5..20) });

    let choices = pick_fighters();

    // Clone the players to avoid borrowing conflicts
    if let (Some(player1), Some(player2)) = (
        characters_list.get(&choices[0]).cloned(),
        characters_list.get(&choices[1]).cloned(),
    ) {
        let mut player1 = player1;
        let mut player2 = player2;
        fight(&mut player1, &mut player2);
    } else {
        println!("Invalid player choices!");
    }

    cltrctoexit::to_exit();
}


fn attack(attacker: &mut Player,  defender:  &mut Player){

   let mut rng = rand::thread_rng();
   let bonus = rng.gen_range(1..6);
   let damage:f32 = (attacker.attack as f32) * bonus as f32;
   let defense: f32 = (defender.defense as f32)/ 100.0;
   let new_health: f32 = defender.health -  (damage * defense);
   

   defender.health = new_health;
   if defender.health > 0.0{
   println!("{} attacked {} for {} damage with bonus {}", attacker.name, defender.name, damage, bonus);
   println!("{} health is now {:.2}", defender.name, defender.health);
   }
   else{
    println!("{} attacked {} for {} damage with bonus {} ", attacker.name, defender.name, damage, bonus);
    println!("{} health is now 0", defender.name);
   }
}


fn fight(player1: &mut Player, player2: &mut Player){

    
    let interval = Duration::from_millis(70);
    let mut next_time = Instant::now() + interval;

    let mut turn = 1;
    loop{
        
        if player1.health <= 0.0{
            println!("{} has Lost", player1.name);
            break;
        }else if player2.health <= 0.0{
            println!("{} has Lost", player2.name);
            break;
        }


        if turn == 1{
            attack(player1, player2);
            turn = 2;
        }else{
            attack(player2, player1);
            turn = 1;
        }
        
       sleep(next_time - Instant::now());
       next_time += interval;
    }

    
}

fn pick_fighters() -> [i32; 2]{
    let mut input: String =  String::new();
    let mut choice:[i32; 2] = [0, 0];
    println!("Choose from fighters below");
    println!("1. Rockky");
    println!("2. ArmStrong");
    println!("3. MayNeel");
    println!("4. Charsiboy");
    println!("5. Daarue");

    
        for x in 0..2{
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            choice[x] = input.trim().parse().expect("Please enter a number");
            println!("You chose {}", choice[x]);
        }

    
    return choice;
}
