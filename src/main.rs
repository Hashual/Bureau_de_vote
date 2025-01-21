use std::collections::BTreeMap as Map;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut candidats: Map<&str, u32> = Map::new();
    candidats.insert("cd1", 0);
    let mut votants = vec!["tux".to_string()];


    loop {

        println!("Listes des commandes : voter, votants, scores");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)?;
        let words = input.split_whitespace().collect::<Vec<_>>();

        

        


        match words[0] {
            "voter" => {
                if words.len() != 3 {
                    println!("nombre d'arguments incorrect");    
                }
                else {
                    let votant = words[1];
                    let candidat = words[2];
                    if !votants.contains(&votant.to_string()){
                        votants.push(votant.to_string());
                        if candidats.contains_key(candidat) {
                            let score = candidats.get_mut(candidat).unwrap();
                            *score += 1;
                            println!("{} a voté pour {}", votant, candidat);
                        }
                        else {
                            println!("{} n'est pas un candidat", candidat);
                        }
                        
                    }
                    else{
                        println!("{} a déjà voté", votant);
                    }
                }
                

            },
            "votants" => {
                println!("");
                println!("nb votants : {}", votants.len());
                println!("");
                for votant in votants.iter() {
                    println!("{}", votant);
                }
                println!("");
            },
            "scores" => {
                println!("");
                for (key, value) in candidats.iter() {
                    println!("{} : {}", key, value);
                }
                println!("");

            },

            _ => {
                println!("Commande inconnue !");
            },
        }
        
    }

    




}