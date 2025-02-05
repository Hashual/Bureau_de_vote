use crate::{domain::VotingMachine, storage::Storage};
use async_trait::async_trait;


pub struct MemoryStore{
    machine: VotingMachine,
}

#[async_trait]
impl Storage for MemoryStore{
    async fn new(machine: VotingMachine) -> anyhow::Result<Self>{
        let memory = Self{
            machine
        };
        Ok(memory)
    }

    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine>{
        Ok(self.machine.clone())
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()>{
        self.machine = machine;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{domain::{BallotPaper, Candidate, Voter, VotingMachine}, storage::Storage, storages::memory::MemoryStore};

    #[tokio::test]
    async fn verifie_que_la_machine_du_get_est_la_meme_que_le_put(){
        let candidats = vec![Candidate("A".to_string()), Candidate("B".to_string())];
        let mut machine = VotingMachine::new(candidats);
        let mut memory = MemoryStore::new(machine.clone()).await.unwrap();
        let machine2 = memory.get_voting_machine().await.unwrap();
        assert_eq!(machine, machine2);
        machine.vote(BallotPaper{voter: Voter("v1".to_string()), candidate: Some(Candidate("A".to_string()))});
        memory.put_voting_machine(machine.clone()).await.unwrap();
        let machine3 = memory.get_voting_machine().await.unwrap();
        assert_eq!(machine, machine3);

    }
}
