use sysinfo::{System, SystemExt};

#[warn(unused_assignments)]
fn main() {
    let mut variable: &str = "HOLA MUNDO";

    variable = "HELLO WORLD";

    println!("{variable} desde Rust or from Rust");

    // ---------------------

    if SystemExt::IS_SUPPORTED {
        let mut system = System::new_all();

        system.refresh_all();

        for disk in system.disks() {
            println!("{}: {} / {} / {}", disk.name(), disk.total_space(), disk.available_space(), disk.used_space());
        }
    }
}
