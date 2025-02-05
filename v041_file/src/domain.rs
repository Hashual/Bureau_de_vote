use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub struct Voter(pub String);

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub struct Candidate(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct Score(pub usize);
#[derive(Clone, Debug, PartialEq)]
pub struct AttendenceSheet(pub Set<Voter>);

#[derive(Clone, Debug, PartialEq)]
pub struct Scoreboard{
    pub scores: Map<Candidate, Score>,
    pub blank_score: Score,
    pub invalid_score:Score,
}

#[derive(Clone)]
pub struct BallotPaper{
    pub voter: Voter,
    pub candidate: Option<Candidate>,
}

#[derive(Debug, PartialEq)]
pub enum VoteOutcome{
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

#[derive(Debug, PartialEq, Clone)]
pub struct VotingMachine {
    voters: AttendenceSheet,
    scoreboard: Scoreboard,
}

impl Scoreboard {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let mut scores = Map::new();
        for candidate in candidates {
            scores.insert(candidate, Score(0));
        }
        Self {
            scores,
            blank_score: Score(0),
            invalid_score: Score(0),
        }
    }
}

impl VotingMachine {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        Self {
            voters: AttendenceSheet(Set::new()),
            scoreboard: Scoreboard::new(candidates),
        }
    }

    pub fn vote (&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
        if self.voters.0.contains(&ballot_paper.voter) {
            return VoteOutcome::HasAlreadyVoted(ballot_paper.voter);
        }
        self.voters.0.insert(ballot_paper.voter.clone());

        if ballot_paper.candidate.is_none() {
            self.scoreboard.blank_score.0 += 1;
            return VoteOutcome::BlankVote(ballot_paper.voter);
        }
        
        let candidate = ballot_paper.candidate.unwrap();
        if self.scoreboard.scores.contains_key(&candidate) {
            self.scoreboard.scores.get_mut(&candidate).unwrap().0 += 1;
            return VoteOutcome::AcceptedVote(ballot_paper.voter, candidate);
        }
        
        self.scoreboard.invalid_score.0 += 1;
        return VoteOutcome::InvalidVote(ballot_paper.voter);
        
    }

    pub fn get_scoreboard(&self) -> &Scoreboard {
        return &self.scoreboard;
    }

    pub fn get_voters(&self) -> &AttendenceSheet {
        return &self.voters;
    }
}
#[cfg(test)]
mod tests {
    use crate::domain::{BallotPaper, Candidate, VoteOutcome, Voter, VotingMachine};

    # [test]
    fn it_works() {
        assert_eq!(1 + 1,2);
    }

    # [test]
    fn test_voter_un_votant_ne_peut_voter_qu_une_seule_fois() {
        let candidats = vec![Candidate("A".to_string()), Candidate("B".to_string())];
        let mut machine = VotingMachine::new(candidats);
        let voter = Voter("Alice".to_string());
        let candidat = Candidate("A".to_string());
        let ballot_paper = BallotPaper{
            voter: voter.clone(),
            candidate: Some(candidat.clone()),
        };
        let result = machine.vote(ballot_paper.clone());
        assert_eq!(result, VoteOutcome::AcceptedVote(voter.clone(), candidat.clone()));
        let result = machine.vote(ballot_paper);
        assert_eq!(result, VoteOutcome::HasAlreadyVoted(voter.clone()));
    }

    # [test]
    fn test_voter_un_votant_peut_voter_pour_un_candidat() {
        let candidats = vec![Candidate("A".to_string()), Candidate("B".to_string())];
        let mut machine = VotingMachine::new(candidats);
        let voter = Voter("Alice".to_string());
        let candidat = Candidate("A".to_string());
        let ballot_paper = BallotPaper{
            voter: voter.clone(),
            candidate: Some(candidat.clone()),
        };
        let result = machine.vote(ballot_paper);
        assert_eq!(result, VoteOutcome::AcceptedVote(voter.clone(), candidat.clone()));
    }

    # [test]
    fn test_voter_un_votant_peut_voter_blanc() {
        let candidats = vec![Candidate("A".to_string()), Candidate("B".to_string())];
        let mut machine = VotingMachine::new(candidats);
        let voter = Voter("Alice".to_string());
        let ballot_paper = BallotPaper{
            voter: voter.clone(),
            candidate: None,
        };
        let result = machine.vote(ballot_paper);
        assert_eq!(result, VoteOutcome::BlankVote(voter.clone()));
    }
    
    # [test]
    fn test_voter_un_votant_peut_voter_nul() {
        let candidats = vec![Candidate("A".to_string()), Candidate("B".to_string())];
        let mut machine = VotingMachine::new(candidats);
        let voter = Voter("Alice".to_string());
        let candidat = Candidate("C".to_string());
        let ballot_paper = BallotPaper{
            voter: voter.clone(),
            candidate: Some(candidat.clone()),
        };
        let result = machine.vote(ballot_paper);
        assert_eq!(result, VoteOutcome::InvalidVote(voter.clone()));
    }

}