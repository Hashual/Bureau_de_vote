use std::collections::BTreeMap as Map;
use v020_configuration::configuration::Configuration;
use clap::Parser;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();

    let mut candidats: Map<&str, u32> = Map::new();
    candidats.insert("blanc", 0 );
    candidats.insert("nul", 0 );

    for candidat in config.candidats.iter() {
        candidats.insert(candidat.as_str(), 0 );
    }

    let mut votants = vec!["tux".to_string()];

    loop {

        println!("Listes des commandes : voter, votants, scores");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)?;
        let words = input.split_whitespace().collect::<Vec<_>>();        

        match words[0] {
            "voter" => {
                if words.len() < 2 || words.len() > 3 {
                    println!("nombre d'arguments incorrect");    
                }
                else {
                    let votant = words[1];
                    if !votants.contains(&votant.to_string()){
                        votants.push(votant.to_string());
                    
                        if words.len() == 2{    
                            println!("Il n'y a pas de candidat, c'est donc un vote blanc");
                            let score = candidats.get_mut("blanc").unwrap();
                            *score += 1
                        }
                        
                        else{
                            let candidat = words[2];    
                            if candidats.contains_key(candidat) {
                                let score = candidats.get_mut(candidat).unwrap();
                                *score += 1;
                                println!("{} a voté pour {}", votant, candidat);
                            }
                            else if !candidats.contains_key(candidat){
                                println!("Le candidat {} n'existe pas, c'est donc un vote null", candidat);
                                let score = candidats.get_mut("nul").unwrap();
                                *score += 1;
                            }   
                        }
                    }

                    else {
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



