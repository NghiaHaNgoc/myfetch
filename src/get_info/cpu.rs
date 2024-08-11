use sysinfo::System;

pub fn cpu(system: &mut System) -> Option<String> {
    system.refresh_cpu_all();
    let cpu = system.cpus().first()?;
    let mut brand = cpu.brand();
    if let Some(index) = brand.find("with") {
        brand = brand[..index].trim();
    }
    Some(brand.to_owned())
}
