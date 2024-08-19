mod svrapp_hbank_communication;
mod svrapp_api_endpoints;

pub use svrapp_hbank_communication::HBankCommunication;
pub use svrapp_api_endpoints::configure_api;

pub mod shared_models {
    pub use h_bank::api_prelude::*;
}


/*
Long-term vision:

1. Current Stage: Direct Integration
   - HBroker-ServerApp uses the HBankCommunication struct.
   - HBankCommunication directly instantiates and uses HBankInterface.
   - All interactions are in-process function calls.

2. Future Stage: API-based Communication
   - HBroker-ServerApp continues to use the HBankCommunication struct (no changes needed here).
   - HBankCommunication is modified internally to make API calls to HBank.
   - HBank implements an API server that exposes the functionality of HBankInterface.

The key benefits of this approach are:

1. Stable Interface: HBroker-ServerApp always interacts with HBankCommunication, regardless of how HBankCommunication is implemented internally.

2. Flexibility: The internal implementation of HBankCommunication can change from direct function calls to API calls without affecting the rest of HBroker-ServerApp.

3. Gradual Transition: You can transition to API-based communication incrementally, one method at a time if needed.

4. Testing Simplicity: It's easier to mock HBankCommunication for testing purposes, as it provides a clear interface.


In the future version:

1. The methods of HBankCommunication remain the same, maintaining the stable interface.
2. Internally, these methods now make HTTP requests to the HBank API.
3. The change is contained within HBankCommunication, so the rest of HBroker-ServerApp doesn't need to change.

This approach gives you the best of both worlds:

1. You can focus on functionality now without worrying about API complexities.
2. You're setting up a structure that will make the transition to API-based communication smoother in the future.
3. The core of HBroker-ServerApp remains stable throughout this potential transition.

By maintaining this consistent abstraction layer through HBankCommunication, I'm future-proofing the architecture while allowing for current simplicity and future flexibility.

*/