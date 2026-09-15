use std::sync::OnceLock;
use tokio::runtime::Runtime;

/// Returns a reference to a shared multi-threaded Tokio runtime.
pub fn get_runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to initialize Tokio runtime")
    })
}

/// Executes a closure within a Tokio runtime context.
pub fn enter_runtime_context<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    match tokio::runtime::Handle::try_current() {
        Ok(_) => f(),
        Err(_) => {
            let rt = get_runtime();
            let _guard = rt.enter();
            f()
        }
    }
}
