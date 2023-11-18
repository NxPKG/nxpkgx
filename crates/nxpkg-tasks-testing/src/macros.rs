#[macro_export]
macro_rules! register {
    () => {
        lazy_static::lazy_static! {
            static ref REGISTER: () = {
                nxpkg_tasks::register();
                include!(concat!(env!("OUT_DIR"), "/register_test_", module_path!(), ".rs"));
            };
        }
    };
}

#[macro_export]
macro_rules! run {
    ($($stmt:tt)+) => {{
        use nxpkg_tasks::NxpkgTasks;
        use nxpkg_tasks_memory::MemoryBackend;
        *REGISTER;
        let tt = NxpkgTasks::new(MemoryBackend::default());
        tt.run_once(async {
            $($stmt)+
            Ok(())
        })
        .await.unwrap();
    }};
}
