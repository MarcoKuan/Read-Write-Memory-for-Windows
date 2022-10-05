use windows::Win32::Foundation::HANDLE;

// Main function that will ask the user for their input (command) in a loop and
// proceeds to read/write/(get a handle) to the process
fn main() {
    let mut id_handle_tup = (&mut 0u32, &mut HANDLE(0));

    loop {
        read_write_procmem_windows::get_cmd(&mut id_handle_tup.0, &mut id_handle_tup.1);
    }
}
