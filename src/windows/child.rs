use std::ffi::OsString;
use std::ffi::OsStr;
use crate::misc::{Result, err_from_str};
use std::os::windows::ffi::OsStrExt;
use std::mem;
use std::ptr;
use winapi::minwindef::{TRUE,FALSE,LPVOID,DWORD};
use winapi::processthreadsapi::{PROCESS_INFORMATION,STARTUPINFOW};
use winapi::winbase::{CREATE_UNICODE_ENVIRONMENT,DETACHED_PROCESS,CREATE_NEW_PROCESS_GROUP};


// let mut p = Command::new(executable);

// use std::os::windows::process::CommandExt;

// p.creation_flags(winapi::winbase::CREATE_NEW_CONSOLE | winapi::winbase::DETACHED_PROCESS);
// let result = p.spawn();

// let child;

pub fn spawn_child_process(executable_path: &OsString) -> Result<PROCESS_INFORMATION> {
    let mut exe = OsStr::new(&executable_path)
            .encode_wide()
            .chain(Some(0u16))
            .collect::<Vec<u16>>();

    // Process Information Out
    let mut pi = PROCESS_INFORMATION {
        hProcess: ptr::null_mut(),
        hThread: ptr::null_mut(),
        dwProcessId: 0,
        dwThreadId: 0,
    };

    // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfoa
    let mut si: STARTUPINFOW = unsafe { mem::zeroed() };
    si.cb = mem::size_of::<STARTUPINFOW>() as DWORD;

    // spawn child
    let result = unsafe {
        kernel32::CreateProcessW(
            exe.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            FALSE,
            CREATE_UNICODE_ENVIRONMENT | DETACHED_PROCESS,
            ptr::null_mut() as LPVOID,
            ptr::null(),
            &mut si,
            &mut pi
        )
    };

    if result == TRUE  {
        Ok(pi)
        // TODO: Properly Closing of the handles
        // unsafe {
        //     kernel32::CloseHandle(pi.hProcess);
        //     kernel32::CloseHandle(pi.hThread);
        // }
    } else {
        Err(err_from_str!("error"))
    }

}
