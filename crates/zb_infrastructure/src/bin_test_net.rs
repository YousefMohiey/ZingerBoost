use windows::Win32::NetworkManagement::IpHelper::{GetIfTable2, FreeMibTable, MIB_IF_TABLE2};

fn main() {
    unsafe {
        let mut table: *mut MIB_IF_TABLE2 = std::ptr::null_mut();
        let res = GetIfTable2(&mut table);
        println!("res: {:?}", res);
        if !table.is_null() {
            FreeMibTable(table as *mut std::ffi::c_void);
        }
    }
}
