/// 'exec's git commands, ending 'gr' and handing off control to 'git'
use anyhow::Result;
use exec::Command;

pub struct ExecGit;

impl ExecGit {
    pub fn new() -> ExecGit {
        ExecGit {}
    }

    pub fn status(&self) -> Result<()> {
        self.git("status", vec![])?;
        Ok(())
    }

    pub fn commit(&self, args: Vec<String>) -> Result<()> {
        self.git("commit", args)?;
        Ok(())
    }

    pub fn pull(&self, args: Vec<String>) -> Result<()> {
        self.git("pull", args)?;
        Ok(())
    }

    // TODO: git mergetool?

    fn git(&self, command: &str, args: Vec<String>) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.arg(command);
        if !args.is_empty() {
            cmd.args(&*args);
        }
        let err = cmd.exec();

        // .exec means we should never get here.
        // our executable should be replaced by git
        Err(err.into())
    }
}