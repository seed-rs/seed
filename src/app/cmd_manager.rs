use futures::future::{abortable, AbortHandle, Future, FutureExt};
use wasm_bindgen_futures::spawn_local;

// ------ CmdManager ------

pub(crate) struct CmdManager;

impl CmdManager {
    pub fn perform_cmd(cmd: impl Future<Output = ()> + 'static) {
        // The future is "leaked" into the JS world as a promise.
        // It's always executed on the next JS tick to prevent stack overflow.
        spawn_local(cmd);
    }

    pub fn perform_cmd_with_handle(cmd: impl Future<Output = ()> + 'static) -> CmdHandle {
        let (cmd, handle) = abortable(cmd);
        // Ignore the error when the future is aborted. I.e. just stop the future execution.
        spawn_local(cmd.map(move |_| ()));
        CmdHandle(handle)
    }
}

// ------ CmdHandle ------

#[derive(Debug)]
pub struct CmdHandle(AbortHandle);

impl Drop for CmdHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}
