use std::fs;
use thiserror::Error;

#[derive(Clone)]
pub struct Env {
    // TODO: These shouldn't need to be public.
    pub fpath: String,
    pub fstr: Option<String>,
}

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("Failed to read environment file")]
    ReadError,
    #[error(transparent)]
    ParseError(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, EnvError>;

impl Env {
    pub fn new(fpath: String) -> Self {
        Env { fpath, fstr: None }
    }

    // TODO: Pull this into some kind of Namer trait?
    pub fn name(&self, dir: String) -> String {
        self.fpath
            .trim_start_matches(dir.as_str())
            .trim_start_matches("/envs/")
            .trim_end_matches(".json")
            .to_owned()
    }

    pub fn load(&mut self) -> Result<()> {
        if self.fstr.is_none() {
            let fstr = fs::read_to_string(self.fpath.clone())
                .map_err(|_| EnvError::ReadError)?;
            self.fstr = Some(fstr);
        }
        Ok(())
    }

    pub fn json(&self) -> Result<serde_json::Value> {
        serde_json::from_str(self.fstr.clone().unwrap().as_str())
            .map_err(|e| EnvError::ParseError(e))
    }
}
