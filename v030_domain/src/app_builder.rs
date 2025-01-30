use std::collections::BTreeMap as Map;
use crate::{configuration::Configuration, domain::{Candidate, Score, Scoreboard, VotingMachine}};

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()>{

    let scoreboard: Scoreboard; 
    let votingMachine = VotingMachine::new(Vec::new());

    
    let blanc = Candidate("blanc".to_string());
    let nul = Candidate("nul".to_string());
    let initial_score = Score(0);

    let mut candidats: Map<Candidate, Score> = Map::new();
    candidats.insert(blanc, initial_score.clone()); 
    candidats.insert(nul,initial_score.clone());

    for candidat in configuration.candidats.iter() {
        candidats.insert(Candidate(candidat.clone()), initial_score.clone());
    }

    let mut votants = vec!["tux".to_string()];

    loop {

        println!("Listes des commandes : voter, votants, scores");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)?;
        let words = input.split_whitespace().collect::<Vec<_>>();        

        match words[0] {
            "voter" => {
                           },
            "votants" => {
                println!("");
                println!("nb votants : {}", votants.len());
                println!("");
                for votant in votants.iter() {
                    println!("{}", votant);
                }
                println!("");
                votingMachine.get_voters();



            },
            "scores" => {
                println!("");
                for (candidat, score) in candidats.iter() {
                    println!("{} : {}", candidat.0, score.0);
                }
                println!("");
                votingMachine.get_scoreboard();

            },

            _ => {
                println!("Commande inconnue !");
            },
        }
        
    }

}
