//! A `CodeExecutor` specialization which uses natively compiled runtime when the wasm to be
//! executed is equivalent to the natively compiled code.

pub use sc_executor::NativeElseWasmExecutor;

// Declare an instance of the native executor named `ExecutorDispatch`. Include the wasm binary as
// the equivalent wasm code.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        // TODO: don't know which runtime to use to picked the alphanet one to make it pass at compilation, to fix
        ternoa_alphanet_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        // TODO: don't know which runtime to use to picked the alphanet one to make it pass at compilation, to fix
        ternoa_alphanet_runtime::native_version()
    }
}
