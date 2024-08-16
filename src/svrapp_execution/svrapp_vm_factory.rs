use crate::execution::vm_trait::VirtualMachine;
use crate::execution::firecracker_vm::FirecrackerVM;
use crate::execution::docker_container::DockerContainer;

pub enum VMType {
    Firecracker,
    Docker,
}

pub struct VMFactory;

impl VMFactory {
    pub async fn create_vm(&self, vm_type: VMType) -> Box<dyn VirtualMachine> {
        match vm_type {
            VMType::Firecracker => Box::new(FirecrackerVM::new().await),
            VMType::Docker => Box::new(DockerContainer::new().await),
        }
    }
}