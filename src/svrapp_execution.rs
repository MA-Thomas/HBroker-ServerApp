pub mod svrapp_engine;
pub mod svrapp_vm_trait;
pub mod svrapp_vm_factory;
pub mod svrapp_wasm_runtime;  // New module for WASM runtime
// pub mod svrapp_firecracker_vm;  // Comment out or remove if no longer needed
// pub mod svrapp_docker_container;  // Comment out or remove if no longer needed

pub use svrapp_engine::ExecutionEngine;
pub use svrapp_vm_trait::VirtualMachine;
pub use svrapp_vm_factory::VMFactory;
pub use svrapp_wasm_runtime::WasmRuntime;  // New WASM runtime
