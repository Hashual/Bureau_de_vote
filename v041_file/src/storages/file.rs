use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use crate::{domain::{self, Candidate, Score, Scoreboard, Voter, VotingMachine}, storage::Storage};


const FILEPATH: &str  = "machine.json";

pub struct FileStore{
    filepath: String,
}

impl FileStore{
    pub async fn create(machine: VotingMachine, filepath: &str) -> anyhow::Result<Self>{
        match File::open(filepath).await  {
            Ok(_) => {
                Ok(Self { filepath: filepath.to_string() })
            }
            Err(_) => {
                let dao_voting_machine:VotingMachineDao = machine.into();
                let vec_machine:Vec<u8>  = serde_json::to_vec(&dao_voting_machine)?;

                let mut myFile = File::create(filepath).await?;
                myFile.write_all(&vec_machine).await?;
                myFile.flush().await?;
                Ok(Self { filepath: filepath.to_string() })
            }
        }
    }
}

#[async_trait::async_trait]
impl Storage for FileStore {
    async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
        return Self::create(machine, FILEPATH).await;
    }
    
    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        let file = File::open(&self.filepath).await?;
        let mut reader = tokio::io::BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).await?;
        let machine: VotingMachineDao = serde_json::from_slice(&buffer)?;
        Ok(machine.into())
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
        let dao_voting_machine: VotingMachineDao = machine.into();
        let vec_machine = serde_json::to_vec(&dao_voting_machine)?;
        let mut file = File::create(&self.filepath).await?;
        file.write_all(&vec_machine).await?;
        file.flush().await?;
        Ok(())
    }

    
}

#[derive(Serialize, Deserialize)]
struct ScoreboardDao {
    scores: BTreeMap<String, usize>,
    blank_score: usize,
    invalid_score: usize,
}

#[derive(Serialize, Deserialize)]
pub struct VotingMachineDao {
    voters: BTreeSet<String>,
    scoreboard: ScoreboardDao
}

impl From<Scoreboard> for ScoreboardDao {
    fn from(scoreboard: Scoreboard) -> Self {
        let mut scores = BTreeMap::new();
        for (k, v) in scoreboard.scores {
            scores.insert(k.0, v.0);
        }
        Self {
            scores,
            blank_score: scoreboard.blank_score.0,
            invalid_score: scoreboard.invalid_score.0,
        }
        
    }
}

impl From<ScoreboardDao> for Scoreboard {
    fn from(scoreboard_dao: ScoreboardDao) -> Self {
        let mut scores = BTreeMap::new();
        for (k, v) in scoreboard_dao.scores {
            scores.insert(Candidate(k), Score(v));
        }
        Self {
            scores,
            blank_score: Score(scoreboard_dao.blank_score),
            invalid_score: Score(scoreboard_dao.invalid_score),
        }
    }
}

impl From<VotingMachine> for VotingMachineDao {
    fn from(machine: VotingMachine) -> Self {
        Self {
            voters: BTreeSet::from_iter(machine.get_voters().0.iter().map(|v| v.0.clone())),
            scoreboard: machine.get_scoreboard().clone().into()
        }
    }
}

impl From<VotingMachineDao> for VotingMachine {
    fn from(machine_dao: VotingMachineDao) -> Self {
        let mut voters = BTreeSet::new();
        for v in machine_dao.voters {
            voters.insert(Voter(v));
        }
        return VotingMachine::recover_from(domain::AttendenceSheet(voters), machine_dao.scoreboard.into());
    }
}