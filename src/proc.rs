use windows::Win32::{
    Foundation::{CloseHandle, BOOL, CHAR, HANDLE},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
            TH32CS_SNAPPROCESS,
        },
        Threading::{OpenProcess, PROCESS_VM_OPERATION, PROCESS_VM_WRITE},
    },
};

// Source example provided by Microsoft: https://learn.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes
// Traverses through the process list
pub fn get_proc_list() -> Vec<(String, u32, HANDLE)> {
    // Return variable: Stores all the necessary data for read/write ("Database" of our process)
    let mut proc_database: Vec<(String, u32, HANDLE)> = Vec::new();
    // tuple that stores (process name, process id, process handle)
    let mut proc_tup: (String, u32, HANDLE);
    // Holds the process information of each
    let mut pe_32: PROCESSENTRY32;
    // The handle to the process (given write access)
    let mut h_proc: HANDLE;
    // Stores a snapshot of all the running processes
    let h_proc_snap: HANDLE;

    unsafe {
        // Init the process entry
        pe_32 = PROCESSENTRY32 {
            dwSize: 0,                   // u32
            cntUsage: 0,                 // u32
            th32ProcessID: 0,            // u32
            th32DefaultHeapID: 0,        // usize
            th32ModuleID: 0,             // u32
            cntThreads: 0,               // u32
            th32ParentProcessID: 0,      // u32
            pcPriClassBase: 0,           // i32
            dwFlags: 0,                  // u32
            szExeFile: [CHAR(0u8); 260], // [CHAR; 260]
        };

        // Take a snapshot of all the processes running
        h_proc_snap = match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
            Ok(snap) => snap,
            Err(e) => panic!("Failed to snapshot correctly: {:?}", e),
        };

        // Set the size of the pe_32 structure
        pe_32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        // Obtain the first process from the snapshot
        // End the program if no process was found
        if Process32First(h_proc_snap, &mut pe_32).eq(&BOOL(0i32)) {
            CloseHandle(h_proc_snap);
            panic!("Error at Process32First");
        }

        // Iterate through the different processes
        loop {
            // Convert the exe file from CHAR(u8) to a String
            let exe_str = pe_32
                .szExeFile
                .iter_mut()
                .take_while(|c| (**c).0 != 0)
                .map(|x| x.0 as char)
                .collect::<String>();

            // Obtain access to the process handle
            h_proc = match OpenProcess(
                PROCESS_VM_WRITE | PROCESS_VM_OPERATION,
                false,
                pe_32.th32ProcessID,
            ) {
                Ok(handle_proc) => handle_proc,
                Err(_) => {
                    // If a process failed to open, it means it might have higher access rights than this program
                    Process32Next(h_proc_snap, &mut pe_32);
                    continue;
                }
            };

            // Initialize our tuple with (process name, process id, process handle)
            proc_tup = (exe_str, pe_32.th32ProcessID, h_proc);

            // Store our tuple into our "return variable"/database
            proc_database.push(proc_tup);

            // End the loop when the whole snapshot has been traversed
            if Process32Next(h_proc_snap, &mut pe_32).eq(&BOOL(0i32)) {
                break;
            }
        }

        // Close handle to the snapshot
        CloseHandle(h_proc_snap);
    }

    // Return the database
    proc_database
}

// Traverse through the vector containing the tuples from get_proc_list and return the tuple from the pid
pub fn get_tuple<'a>(
    list: &'a Vec<(String, u32, HANDLE)>,
    desired_pid: &u32,
) -> &'a (String, u32, HANDLE) {
    list.iter().find(|tup| (*tup).1 == *desired_pid).unwrap()
}
