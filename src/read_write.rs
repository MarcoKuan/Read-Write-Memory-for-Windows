use core::ffi::c_void;
use std::ptr::null_mut;
use windows::Win32::{
    Foundation::HANDLE,
    System::Diagnostics::{Debug::WriteProcessMemory, ToolHelp::Toolhelp32ReadProcessMemory},
};

// Reads a certain amount of bytes in an address, given the pid, and outputs the read bytes
pub fn read_bytes(pid: u32, base_addr: u64, num_bytes: usize) -> Vec<u8> {
    let mut buf_read: Vec<u8> = vec![0u8; num_bytes];

    unsafe {
        // Reads from the address of a pid
        Toolhelp32ReadProcessMemory(
            pid,                                  // u32
            base_addr as *const c_void,           // *const c_void
            buf_read.as_mut_ptr() as *mut c_void, // *mut c_void
            num_bytes,                            // usize
            null_mut(),                           // *mut usize
        );
    }

    // Returns "num_bytes" *read* from "base_addr" of the "pid"
    buf_read
}

// Writes certain bytes to an address when given the handle to the process
pub unsafe fn write_bytes(h_proc: HANDLE, base_addr: u64, buf_write: Vec<u8>) {
    WriteProcessMemory(
        h_proc,                              // P0 (generic)
        base_addr as *const c_void,          // *const c_void
        buf_write.as_ptr() as *const c_void, // *const c_void
        buf_write.len(),                     // usize
        Some(null_mut()),                    // Option<*mut usize>
    );
}
