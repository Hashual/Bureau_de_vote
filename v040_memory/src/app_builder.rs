use crate::{configuration::Configuration, domain::{BallotPaper, Candidate, VoteOutcome, Voter, VotingMachine}};

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()>{

    let mut candidats: Vec<Candidate> =  vec![];
    
    for candidat in configuration.candidats.iter() {
        candidats.push(Candidate(candidat.clone()));
    }
    
    let mut votingMachine = VotingMachine::new(candidats);

    loop {
        println!("Listes des commandes : voter, votants, scores");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)?;
        let words = input.split_whitespace().collect::<Vec<_>>();        

        match words[0] {
            "voter" => {
                let candidat: Option<Candidate> = if words.len() == 2 {
                    None
                } else {
                    Some(Candidate(words[2].to_string()))
                };

                let ballotPaper: BallotPaper =  BallotPaper{
                    voter : Voter(words[1].to_string()),
                    candidate : candidat,
                };
                let result = votingMachine.vote(ballotPaper);
                match result {
                    VoteOutcome::AcceptedVote(voter, candidate) => {
                        println!("{} a voté pour {}", voter.0, candidate.0);
                    },
                    VoteOutcome::BlankVote(voter) => {
                        println!("{} a voté blanc", voter.0);
                    },
                    VoteOutcome::InvalidVote(voter) => {
                        println!("{} a voté nul", voter.0);
                    },
                    VoteOutcome::HasAlreadyVoted(voter) => {
                        println!("{} a déjà voté", voter.0);
                    },
                };

            },
            "votants" => {
                println!("");
                for voter in votingMachine.get_voters().0.iter() {
                    println!("{}", voter.0);
                }



            },
            "scores" => {
                println!("");
                println!("Votes blancs : {}", votingMachine.get_scoreboard().blank_score.0);
                println!("Votes nuls : {}", votingMachine.get_scoreboard().invalid_score.0);
                for (candidat, score) in votingMachine.get_scoreboard().scores.iter() {
                    println!("{} : {}", candidat.0, score.0);
                }
                println!("");

            },

            _ => {
                println!("Commande inconnue !");
            },
        }
        
    }

}
