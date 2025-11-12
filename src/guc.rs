use crate::guc::ffi::CString;
use anyhow::Result;
use pgrx::*;

pub static PGLATER_SOCKET_URL: GucSetting<Option<CString>> =
    GucSetting::<Option<CString>>::new(None);

// initialize GUCs
pub fn init_guc() {
    GucRegistry::define_string_guc(
        c"pglater.host",
        c"unix socket url for Postgres",
        c"unix socket path to the Postgres instance. Optional. Can also be set in environment variable PGLATER_SOCKET_URL.",
        &PGLATER_SOCKET_URL,
        GucContext::Suset, GucFlags::default());
}

// for handling of GUCs that can be error prone
#[derive(Debug)]
pub enum PglaterGUC {
    Host,
}

/// a convenience function to get this project's GUCs
pub fn get_guc(guc: PglaterGUC) -> Option<String> {
    let val = match guc {
        PglaterGUC::Host => PGLATER_SOCKET_URL.get(),
    };
    if let Some(cstr) = val {
        if let Ok(s) = handle_cstr(cstr) {
            Some(s)
        } else {
            error!("failed to convert CStr to str");
        }
    } else {
        info!("no value set for GUC: {:?}", guc);
        None
    }
}

#[allow(dead_code)]
fn handle_cstr(cstr: CString) -> Result<String> {
    if let Ok(s) = cstr.to_str() {
        Ok(s.to_owned())
    } else {
        Err(anyhow::anyhow!("failed to convert CStr to str"))
    }
}
