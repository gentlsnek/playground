use std::thread::sleep;
use std::time::{Duration, Instant};
use rand::Rng;
use std::io;


pub struct Player{
    name: String,
    health: f32,
    attack: i32,
    defense: i32,
}

fn main(){
    let mut rng = rand::thread_rng();


    let p1_attack = rng.gen_range(1..20);
    let p1_defense = rng.gen_range(5..20);
    let p2_attack = rng.gen_range(1..20);
    let p2_defense = rng.gen_range(5..20);
    let p3_attack = rng.gen_range(1..20);
    let p3_defense = rng.gen_range(5..20);
    let p4_attack = rng.gen_range(1..20);
    let p4_defense = rng.gen_range(5..20);
    

    let mut _player1 = Player{
        name: String::from("Rockky"),
        health: 100.0,
        attack: p1_attack,
        defense: p1_defense,
    };
    

    let mut _player2 = Player{
        name: String::from("ArmStrong"),
        health: 100.0,
        attack: p2_attack,
        defense: p2_defense,
    };

    let mut _player3 = Player{
        name: String::from("MayNeel"),
        health: 100.0,
        attack: p3_attack,
        defense: p3_defense,
    };

    let mut _player4 = Player{
        name: String::from("Charsiboy"),
        health: 100.0,
        attack: p4_attack,
        defense: p4_defense,
    };

   // let choice =  pick_fighters();

    fight(&mut _player3, &mut _player4);

}



fn attack(attacker:  &mut Player,  defender:  &mut Player){

   let mut rng = rand::thread_rng();
   let bonus = rng.gen_range(1..5);
   let damage:f32 = (attacker.attack as f32) * bonus as f32;
   let defense: f32 = (defender.defense as f32)/ 100.0;
   let new_health: f32 = defender.health -  (damage * defense);
   

   defender.health = new_health;
   println!("{} attacked {} for {} damage with bonus {}", attacker.name, defender.name, damage, bonus);
   println!("{} health is now {}", defender.name, defender.health);
}


fn fight(player1: &mut Player, player2: &mut Player){

    
    let interval = Duration::from_millis(70);
    let mut next_time = Instant::now() + interval;

    let mut turn = 1;
    loop{
        if turn == 1{
            attack(player1, player2);
            turn = 2;
        }else{
            attack(player2, player1);
            turn = 1;
        }
        if player1.health <= 0.0{
            println!("{} has died", player1.name);
            break;
        }else if player2.health <= 0.0{
            println!("{} has died", player2.name);
            break;
        }
       sleep(next_time - Instant::now());
       next_time += interval;
    }
}

/*fn pick_fighters() -> [i32; 2]{
    let mut input: String =  String::new();
    let mut choice:[i32; 2] = [0, 0];
    println!("Choose from fighters below");
    println!("1. Rockky");
    println!("2. ArmStrong");
    println!("3. MayNeel");
    println!("4. Charsiboy");

    
        for x in 0..2{
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            choice[x] = input.trim().parse().expect("Please enter a number");
            println!("You chose {}", choice[x]);
        }

    
    return choice;
}

fn fighter_choice(fighter: i32) -> &mut Player{

    match fighter {
        1 => player1,
        2 => player2,
        3 => player3,
        4 => player4,
    }
}
    */