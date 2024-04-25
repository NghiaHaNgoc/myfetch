use sysinfo::Disks;

const ONE_GIB: u64 = 1_073_741_824;

pub fn disk() -> Option<String> {
    let disks = Disks::new_with_refreshed_list();
    let mut current_disk = None;
    for disk in disks.list() {
        if disk.mount_point().to_str()?.eq("/") {
            current_disk = Some(disk);
            break;
        }
    }
    let current_disk = current_disk?;
    let disk_total = current_disk.total_space();
    let disk_used = disk_total - current_disk.available_space();
    Some(format!(
        "{:.2} GiB / {:.2} GiB ({}%)",
        disk_used as f32 / ONE_GIB as f32,
        disk_total as f32 / ONE_GIB as f32,
        disk_used * 100 / disk_total
    ))
}
