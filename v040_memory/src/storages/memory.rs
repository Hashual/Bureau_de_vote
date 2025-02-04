use crate::{domain::VotingMachine, storage::Storage};
use async_trait::async_trait;


pub struct MemoryStore{

}

#[async_trait]
impl Storage for MemoryStore{
    async fn new(machine: VotingMachine) -> anyhow::Result<Self>{
        Ok(MemoryStore{})
    }

    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine>{
        Ok(VotingMachine::new(vec![]))
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()>{
        Ok(())
    }
}
