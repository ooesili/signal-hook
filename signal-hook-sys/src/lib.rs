//! Low-level internals of [`signal-hook`][https://docs.rs/signal-hook].
//!
//! This crate contains some internal APIs, split off to a separate crate for technical reasons. Do
//! not use directly. There are no stability guarantees, no documentation and you should use
//! `signal-hook` directly.

#[doc(hidden)]
pub mod internal {
    use libc::{abort, pid_t, siginfo_t, uid_t};

    // Careful: make sure the signature and the constants match the C source
    extern "C" {
        fn sighook_signal_origin(info: *const siginfo_t, pid: *mut pid_t, uid: *mut uid_t) -> u8;
    }

    const ORIGIN_UNKNOWN: u8 = 0;
    const ORIGIN_PROCESS: u8 = 1;
    const ORIGIN_KERNEL: u8 = 2;

    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum Cause {
        User,
        Queue,
        MesgQ,
        Exited,
        Killed,
        Dumped,
        Trapped,
        Stopped,
        Continued,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum Origin {
        Unknown,
        Kernel,
        Process {
            pid: pid_t,
            uid: uid_t,
            cause: Cause,
        }
    }

    impl Origin {
        pub fn extract(info: &siginfo_t) -> Self {
            let mut pid: pid_t = 0;
            let mut uid: uid_t = 0;
            let origin = unsafe { sighook_signal_origin(info, &mut pid, &mut uid) };
            match origin {
                ORIGIN_UNKNOWN => Origin::Unknown,
                ORIGIN_KERNEL => Origin::Kernel,
                // TODO
                ORIGIN_PROCESS => Origin::Process { pid, uid, cause: Cause::User },
                _ => unsafe { abort() }, // Not unreachable. Not async-signal-safe.
            }
        }
    }
}
