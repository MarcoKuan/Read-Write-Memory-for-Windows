use core::ffi::c_void;
use std::ptr::null_mut;
use windows::Win32::{
    Foundation::HANDLE,
    System::Diagnostics::{Debug::WriteProcessMemory, ToolHelp::Toolhelp32ReadProcessMemory},
};

/// Reads a certain amount of bytes in an address, given the pid, and outputs the read bytes
/// This program is similar to gdb's "x" command on an address
///
/// Returns a Vec<u8> (bytes) that holds the byte instructions on that address
///
/// # Arguments
///
/// * `pid`         - A reference to a u32 that holds the process id
/// * `base_addr`   - A reference to a u64 that holds the address that will be read from
/// * `num_bytes`   - A reference to a usize that holds the number of bytes to be read from
///
/// # Examples
///
/// ```
/// // The result will output the bytes read at the given pid and addr of that process
/// use read_write_procmem_windows::read_write;
///
/// let pid = 100u32;
/// let addr = 0xDEADBEEFu64;
/// let len = 10usize;
///
/// let bytes_read: Vec<u8> = read_write::read_bytes(&pid, &addr, &len);
/// ```
///
/// Refer to [Toolhelp32ReadProcessMemory](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-toolhelp32readprocessmemory) and [Toolhelp32ReadProcessMemory](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/ToolHelp/fn.Toolhelp32ReadProcessMemory.html)
pub fn read_bytes(pid: &u32, base_addr: &u64, num_bytes: &usize) -> Vec<u8> {
    // Stores the bytes that were read into the vector
    let mut buf_read: Vec<u8> = vec![0u8; *num_bytes];

    unsafe {
        // Reads from the address of a pid
        Toolhelp32ReadProcessMemory(
            *pid,                                 // u32
            (*base_addr) as *const c_void,        // *const c_void
            buf_read.as_mut_ptr() as *mut c_void, // *mut c_void
            *num_bytes,                           // usize
            null_mut(),                           // *mut usize
        );
    }

    // Returns "num_bytes" *read* from "base_addr" of the "pid"
    buf_read
}

/// Writes certain bytes to an address when given the handle to the process
///
/// Returns nothing
///
/// # Arguments
///
/// * `h_proc`      - A reference to a [HANDLE](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HANDLE.html) that holds the handle to the process for writing to the bytes
/// * `base_addr`   - A reference to a u64 that holds the address that will be written from
/// * `buf_write`   - A reference to a Vec<u8> that holds the the address to be written
///
/// # Examples
///
/// ```
/// // The result will output the bytes read at the given pid and addr of that process
/// use read_write_procmem_windows::read_write;
/// use read_write_procmem_windows::proc;
///
/// let database = proc::get_proc_list();
/// let h_proc = database[0].2;
/// let addr = 0xDEADBEEFu64;
/// let w_bytes: Vec<u8> = vec![0x90u8; 5];
///
/// unsafe {
///     read_write::write_bytes(&h_proc, &addr, &w_bytes);
/// }
/// ```
///
/// Refer to [WriteProcessMemory](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory) and [WriteProcessmemory](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Diagnostics/Debug/fn.WriteProcessMemory.html)
pub unsafe fn write_bytes(h_proc: &HANDLE, base_addr: &u64, buf_write: &Vec<u8>) {
    WriteProcessMemory(
        *h_proc,                             // P0 (generic)
        (*base_addr) as *const c_void,       // *const c_void
        buf_write.as_ptr() as *const c_void, // *const c_void
        buf_write.len(),                     // usize
        Some(null_mut()),                    // Option<*mut usize>
    );
}
