use std::fmt::Display;

#[cfg(windows)]
pub struct PandoraExitStatus(pub(crate) u32);

#[cfg(windows)]
impl Display for PandoraExitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 & 0x80000000 != 0 {
            f.write_fmt(format_args!("exitcode={:#x}", self.0))
        } else {
            f.write_fmt(format_args!("exitcode={}", self.0))
        }
    }
}

#[cfg(unix)]
pub struct PandoraExitStatus(libc::c_int);

#[cfg(unix)]
impl Display for PandoraExitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        compile_error!("todo");
    }
}
