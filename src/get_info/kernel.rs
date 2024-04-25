use sysinfo::System;

pub fn kernel() -> Option<String> {
    System::kernel_version()
}
