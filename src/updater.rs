pub mod pnpm;

pub trait Updater {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> String;

    fn list(&self) -> Option<Vec<Package>>;
}

pub struct Package {
    pub name: String,
    pub current_version: String,
    pub latest_version: String,
}
