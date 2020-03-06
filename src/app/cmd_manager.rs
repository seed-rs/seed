use futures::future::{abortable, AbortHandle, Future, FutureExt};
use wasm_bindgen_futures::spawn_local;

// ------ CmdManager ------

pub struct CmdManager;

impl CmdManager {
    pub fn perform_cmd(cmd: impl Future<Output = ()> + 'static) {
        spawn_local(cmd);
    }

    pub fn perform_cmd_with_handle(cmd: impl Future<Output = ()> + 'static) -> CmdHandle {
        let (cmd, handle) = abortable(cmd);
        spawn_local(cmd.map(move |_| ()));
        CmdHandle(handle)
    }
}

// ------ CmdHandle ------

pub struct CmdHandle(AbortHandle);

impl Drop for CmdHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}
