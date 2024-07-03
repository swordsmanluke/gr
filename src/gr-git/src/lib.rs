use std::error::Error;
use std::process::Command;

pub struct Git {
}

impl Git {
    pub fn new() -> Git {
        Git {}
    }

    pub fn branch(&self, args: &str) -> Result<String, Box<dyn Error>> {
        self.git("branch", args)
    }

    fn git(&self, command: &str, args: &str) -> Result<String, Box<dyn Error>> {
        // Execute the git executable with the given command and arguments
        // and return the output
        let split_args = args.split(" ").collect::<Vec<&str>>();
        let mut g = Command::new("git");
        let mut cmd = g.arg(command);

        if split_args.len() == 0 {
            cmd = cmd.args(split_args);
        }


        let output = cmd.output().expect(format!("Failed to execute {} {} {}", "git", command, args).as_str()).stdout;

        // Convert the output to a string and return it
        Ok(String::from_utf8(output)?.trim().to_string())
    }
}