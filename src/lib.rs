/*
 * Copyright 2020 fsyncd, Berlin, Germany.
 * Additional material, copyright of the containerd authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! A Rust wrapper around the Linux ip(8) command. Show / manipulate routing, network devices, interfaces and tunnels.

use snafu::{ensure, OptionExt, ResultExt, Snafu};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;
use std::{env, io};
use tokio::process::Command;
use tokio::time::timeout;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Ip command error: {}", source))]
    CommandError { source: io::Error },

    #[snafu(display("Ip command failed, stdout: \"{}\", stderr: \"{}\"", stdout, stderr))]
    CommandFailedError { stdout: String, stderr: String },

    #[snafu(display("Ip command not found in PATH"))]
    CommandNotFoundError {},

    #[snafu(display("Ip command timed out: {}", source))]
    CommandTimeoutError { source: tokio::time::Elapsed },

    #[snafu(display("Unable to spawn process: {}", source))]
    SpawnError { source: io::Error },
}

pub struct IpCommand {
    command: PathBuf,
    timeout: Duration,
    namespace: Option<PathBuf>,
}

impl IpCommand {
    /// Create a new ip(8) command client.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            command: Self::ip_binary_path().context(CommandNotFoundError {})?,
            timeout: Duration::from_millis(5_000),
            namespace: None,
        })
    }

    /// Return the current version of the ip(8) command.
    pub async fn version(&self) -> Result<String, Error> {
        self.command(&["-Version".into()], false).await
    }

    /// Create a new IpCommand instance for the specified network namespace.
    pub fn with_namespace(&self, namespace: &Path) -> Self {
        Self {
            command: self.command.clone(),
            timeout: self.timeout,
            namespace: Some(PathBuf::from(namespace)),
        }
    }

    async fn command(&self, args: &[String], combined_output: bool) -> Result<String, Error> {
        let args = self.concat_args(args)?;
        let process = Command::new(&self.command)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context(SpawnError {})?;

        let result = timeout(self.timeout, process.wait_with_output())
            .await
            .context(CommandTimeoutError {})?
            .context(CommandError {})?;

        let stdout = String::from_utf8(result.stdout.clone()).unwrap();
        let stderr = String::from_utf8(result.stderr.clone()).unwrap();
        ensure!(
            result.status.success(),
            CommandFailedError {
                stdout: stdout,
                stderr: stderr
            }
        );

        Ok(if combined_output {
            let mut combined = String::new();
            combined.push_str(&stdout);
            combined.push_str(&stderr);
            combined
        } else {
            stdout
        })
    }

    fn concat_args(&self, args: &[String]) -> Result<Vec<String>, Error> {
        let mut combined_args: Vec<String> = vec!["-json".into()];
        if let Some(namespace) = &self.namespace {
            combined_args.push("-netns".into());
            combined_args.push(namespace.canonicalize().unwrap().to_string_lossy().into());
        }
        combined_args.append(&mut Vec::from_iter(args.iter().cloned()));
        Ok(combined_args)
    }

    fn ip_binary_path() -> Option<PathBuf> {
        env::var_os("PATH").and_then(|paths| {
            env::split_paths(&paths)
                .filter_map(|dir| {
                    let full_path = dir.join("ip");
                    if full_path.is_file() {
                        Some(full_path)
                    } else {
                        None
                    }
                })
                .next()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[tokio::test]
    async fn test_version() {
        let ip_command = IpCommand::new().unwrap();
        let version = ip_command.version().await.unwrap();
        assert!(Regex::new(r"ip utility, iproute2-ss\d{6}")
            .unwrap()
            .is_match(&version));
    }
}