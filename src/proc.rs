use std::collections::BTreeMap;

use windows::Win32::{
    Foundation::{CloseHandle, BOOL, CHAR, HANDLE},
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
    },
};

// Source example provided by Microsoft: https://learn.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes
// Traverses through the process list
pub fn get_proc_list() -> BTreeMap<String, u32> {
    // B-TreeMap used to store (exe_name, process_id), sorted in alphabetical order
    let mut exe_id_map: BTreeMap<String, u32> = BTreeMap::new();

    unsafe {
        // Holds the process information of each
        let mut pe_32: PROCESSENTRY32 = PROCESSENTRY32 {
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

        // Stores a snapshot of all the running processes
        let h_proc_snap: HANDLE = match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
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

            // Store (exe_str, process_id) into "exe_id_map" B-TreeMap
            exe_id_map.insert(exe_str, pe_32.th32ProcessID);

            // End the loop when the whole snapshot has been traversed
            if Process32Next(h_proc_snap, &mut pe_32).eq(&BOOL(0i32)) {
                break;
            }
        }
    }

    // Return the BTreeMap
    exe_id_map
}
