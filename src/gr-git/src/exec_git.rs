/// 'exec's git commands, ending 'gr' and handing off control to 'git'
use std::error::Error;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use exec::Command;

pub struct ExecGit;

impl ExecGit {
    pub fn new() -> ExecGit {
        ExecGit {}
    }

    pub fn commit(&self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.git("commit", args)?;
        // we should never get here.
        Ok(())
    }

    fn git(&self, command: &str, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let err = Command::new("git")
            .arg(command)
            .args(&*args)
            .exec();

        // .exec means we should never get here.
        // our executable should be replaced by git
        Err(err.into())
    }
}