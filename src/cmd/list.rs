use std::collections::HashMap;

use crate::updater::{pnpm::Pnpm, Package, Updater};

pub fn run() {
    let updaters = vec![Pnpm::new("pnpm")];

    let updates = updaters.iter().fold(
        HashMap::<String, Vec<Package>>::new(),
        |mut acc, updater| {
            if let Some(pkgs) = updater.list() {
                acc.insert(updater.name(), pkgs);
            }

            return acc;
        },
    );

    for (package_manager, packages) in updates {
        let (a, b) = package_manager.split_at(1);

        println!(
            "{} has {} outdated packages",
            a.to_uppercase() + b,
            packages.len()
        );

        for pkg in packages {
            println!(
                "{}: {} -> {}",
                pkg.name, pkg.current_version, pkg.latest_version
            );
        }
    }
}
