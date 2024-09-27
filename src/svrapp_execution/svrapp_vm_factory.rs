use crate::svrapp_execution::svrapp_vm_trait::VirtualMachine;
use crate::svrapp_execution::svrapp_wasm_runtime::WasmRuntime;

pub enum VMType {
    Wasm,
    // Firecracker,  // Comment out or remove if no longer needed
    // Docker,  // Comment out or remove if no longer needed
}

pub struct VMFactory;

impl VMFactory {
    pub fn create_vm(&self, vm_type: VMType) -> Box<dyn VirtualMachine> {
        match vm_type {
            VMType::Wasm => Box::new(WasmRuntime::new()),
            // VMType::Firecracker => Box::new(FirecrackerVM::new()),  // Remove if no longer needed
            // VMType::Docker => Box::new(DockerContainer::new()),  // Remove if no longer needed
        }
    }
}
