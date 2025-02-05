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
