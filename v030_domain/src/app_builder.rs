use std::collections::BTreeMap as Map;
use crate::{configuration::Configuration, domain::{BallotPaper, Candidate, Score, Scoreboard, Voter, VotingMachine}};

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()>{

    let mut votingMachine = VotingMachine::new(Vec::new());
    let initial_score = Score(0);

    let mut scoreboard: Scoreboard= Scoreboard{
        scores: Map::new(),
        blank_score: initial_score.clone(),
        invalid_score: initial_score.clone(),
    };

    for candidat in configuration.candidats.iter() {
        scoreboard.scores.insert(Candidate(candidat.clone()), initial_score.clone());
    }

    let mut votants = vec!["tux".to_string()];

    loop {

        println!("Listes des commandes : voter, votants, scores");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)?;
        let words = input.split_whitespace().collect::<Vec<_>>();        

        match words[0] {
            "test" => {
                
            },

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
                for (candidat, score) in scoreboard.scores.iter() {
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
