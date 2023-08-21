use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use subprocess::Exec;

use crate::updater::Package;

use super::Updater;

#[derive(Debug)]
pub struct Pnpm {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PnpmPackage {
    current: String,
    latest: String,
    wanted: String,
    is_deprecated: bool,
    dependency_type: String,
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
        let output = match Exec::shell(format!("{} outdated --global --json", self.name))
            .capture()
            .map(|cmd| cmd.stdout_str())
        {
            Ok(output) => output,
            _ => return None,
        };

        match from_str::<HashMap<String, PnpmPackage>>(&output) {
            Ok(pkgs) => Some(
                pkgs.iter()
                    .map(|(name, pkg)| -> Package {
                        Package {
                            name: name.to_string(),
                            current_version: pkg.current.to_string(),
                            latest_version: pkg.latest.to_string(),
                        }
                    })
                    .collect(),
            ),
            _ => None,
        }
    }
}
