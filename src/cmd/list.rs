use crate::updater::{pnpm::Pnpm, Updater};

pub fn run() {
    let pnpm = Pnpm::new("pnpm");

    match pnpm.list() {
        Some(pkgs) => pkgs
            .iter()
            .for_each(|pkg| println!("{}: {}", pkg.name, pkg.current_version)),
        None => panic!("No packages found"),
    }
}
