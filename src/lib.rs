pub mod proc;
pub mod read_write;
use std::io::Write;
use windows::Win32::Foundation::HANDLE;

/// Asks for the user's input and proceeds to run commands based on the options given
///
/// # Arguments
/// * `pid` - The process id that the user will read/write/attach to
/// * `h_proc` - The handle to the process which is used for writing to the specified process (in pid)
///
/// # User input:
/// |                              |           |    Format            |                                               |             Example 1         |          Example 2            |
/// |------------------------------|-----------|----------------------|-----------------------------------------------|-------------------------------|-------------------------------|
/// |           Name               |  \[cmd\]  |      \[addr\]        |                 \[w/r bytes\]                 |                               |                               |
/// |       (P)rocess Attach       |     P     | [process in decimal] |                                               | P 103                         |                               |
/// |       Process (R)ead         |     W     | [Address in dec/hex] |       [How many bytes to read in dec]         | R 0xDEADBEEF 5                | R 3735928559 5                |
/// |       Process (W)rite        |     R     | [Address in dec/hex] |  [String of byte code instructions to write]  | W 0xDEADBEEF \xEF\xBE\xAD\xDE | W 3735928559 \xEF\xBE\xAD\xDE |
pub fn get_cmd(pid: &mut u32, h_proc: &mut HANDLE) {
    // User input string
    let mut buf = String::new();
    // Holds the three arguments [cmd] [addr] [w/r bytes]
    let cmd_arr: Vec<&str>;
    // Holds [cmd]: P/R/W
    let cmd: &str;
    // Holds [addr]: Hex/Dec
    let addr: u64;
    // Holds [w/r bytes]: "\x90\x00" (Hex) or "4" (Dec)
    let wr_bytes: &str;

    // Get user input
    match std::io::stdin().read_line(&mut buf) {
        Ok(it) => it,
        Err(err) => panic!("{}", err),
    };

    // Take the command and split them into sections
    // [cmd] [addr] [w_bytes]
    cmd_arr = buf.trim().split(" ").collect::<Vec<&str>>();

    // Arg 0: Get the commmand of the program
    cmd = cmd_arr.get(0).unwrap();
    // Arg 1: Get the address/pid of the program
    // Determines whether to convert the string to a:
    // Hex: Starts with 0x
    // Dec: Other input
    let temp_addr: &str = cmd_arr.get(1).unwrap().trim();
    addr = match temp_addr.starts_with("0x") {
        true => u64::from_str_radix(temp_addr.trim_start_matches("0x"), 16).unwrap(),
        false => u64::from_str_radix(temp_addr, 10).unwrap(),
    };

    // Short-circuit to ask for another input (Attaches to process)
    // [Note: addr is process id b/c it is the 1st argument]
    // [Counting from 0th, 1st, 2nd... arguments]
    if cmd == "P" {
        // Updates the process list and tries to get the handle with
        // open_process_access (Write) to the process id specified
        let proc_list = proc::get_proc_list().to_owned();
        let name_id_handle_tup = proc::get_tuple(&proc_list, &(addr as u32));

        // Set the values of and ask the user for the next command
        *pid = name_id_handle_tup.1;
        *h_proc = name_id_handle_tup.2;
        return;
    }

    // Arg 2: Holds the Write Bytes/Number of Read bytes
    wr_bytes = &cmd_arr.get(2).unwrap().trim();

    // Parse the command:
    // [W/R/P] [u32/u32/u32] [String/u32/NA]
    if cmd == "W" {
        unsafe {
            // Converts the bytes that will be written into a Vector that will be used
            // to write into the addr provided by the user
            let temp_buf = wr_bytes
                .trim_start_matches("\\x")
                .split("\\x")
                .collect::<Vec<&str>>();
            println!("{:?}", temp_buf);
            let buf_write = temp_buf
                .iter()
                .map(|b| u8::from_str_radix(*b, 16).unwrap())
                .collect::<Vec<u8>>();

            // Writes certain bytes into a specific address
            read_write::write_bytes(&h_proc, &addr, &buf_write);

            // Output (Flushing to output before the next input)
            println!("Write @: {:x}", addr);
            let _ = std::io::stdout().flush();
        }
    } else if cmd == "R" {
        // Store the data involved in reading the "number of bytes" and "bytes read"
        let num_bytes = u64::from_str_radix(wr_bytes, 10).unwrap() as usize;
        let buffer = read_write::read_bytes(pid, &addr, &num_bytes);

        // Output address read and number of bytes in hex value to the terminal
        println!("Read @: {:x}", addr);
        print!("Hex: ");
        buffer.iter().for_each(|b| print!("{:x} ", b));

        // Flushing and printing a new line to make the terminal easier to track/read
        println!();
        let _ = std::io::stdout().flush();
    } else {
        // Invalid input
        get_cmd(pid, h_proc)
    }
    // Jump back on the loop to re-read on the same PID
    // Return the current pid and handle to the process
}
