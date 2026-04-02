use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use anyhow::Result;
use async_trait::async_trait;
use crate::tools::Tool;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillDefinition {
    pub name: String,
    pub description: String,
    pub command: String,
}

pub struct SkillTool {
    definition: SkillDefinition,
}

impl SkillTool {
    pub fn new(definition: SkillDefinition) -> Self {
        Self { definition }
    }

    fn substitute_args(&self, command: &str, args: &Value) -> String {
        let mut result = command.to_string();
        if let Some(obj) = args.as_object() {
            for (key, val) in obj {
                let placeholder = format!("{{{{{}}}}}", key);
                let val_str = val.as_str().unwrap_or("");
                result = result.replace(&placeholder, val_str);
            }
        }
        result
    }
}

#[async_trait]
impl Tool for SkillTool {
    fn name(&self) -> &str { &self.definition.name }
    fn description(&self) -> &str { &self.definition.description }
    async fn run(&self, args: Value) -> Result<String> {
        let final_command = self.substitute_args(&self.definition.command, &args);
        let output = Command::new("sh").arg("-c").arg(final_command).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}

pub fn load_skills_from_dir(dir: &str) -> Result<Vec<Box<dyn Tool>>> {
    let mut skills = Vec::new();
    let path = Path::new(dir);
    if !path.exists() {
        return Ok(skills);
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_some(|ext| ext == "json") {
            let content = fs::read_to_string(&path)?;
            let definition: SkillDefinition = serde_json::from_str(&content)?;
            skills.push(Box::new(SkillTool::new(definition)) as Box<dyn Tool>);
        }
    }
    Ok(skills)
}

trait OptionExt {
    fn and_some<F>(self, f: F) -> bool where F: FnOnce(&std::ffi::OsStr) -> bool;
}

impl OptionExt for Option<&std::ffi::OsStr> {
    fn and_some<F>(self, f: F) -> bool where F: FnOnce(&std::ffi::OsStr) -> bool {
        match self {
            Some(s) => f(s),
            None => false,
        }
    }
}
