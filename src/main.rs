use read_write_procmem_windows::proc::get_proc_list;

fn main() {
    println!("{:#?}", get_proc_list());
}
