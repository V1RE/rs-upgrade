use subprocess::Exec;

use crate::updater::Package;

use super::Updater;

#[derive(Debug)]
pub struct Pnpm {
    name: String,
}

impl Updater for Pnpm {
    fn new(name: &'static str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn list(&self) -> Option<Vec<super::Package>> {
        let output = Exec::shell(format!("{} outdated --global --json", self.name))
            .capture()
            .map(|cmd| cmd.stdout_str());

        return match output {
            Ok(out) => {
                let name = out.replace("\n", "");
                Some(vec![Package {
                    name,
                    current_version: "lol".to_owned(),
                    latest_version: "lol".to_string(),
                }])
            }
            Err(err) => {
                println!("Error: {}", err);
                None
            }
        };
    }
}
