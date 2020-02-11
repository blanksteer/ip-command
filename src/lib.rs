/*
 * Copyright 2020 fsyncd, Berlin, Germany.
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

use crate::command::*;
use futures::ready;
use futures::task::{Context, Poll};
use futures::Stream;
use snafu::{ensure, OptionExt, ResultExt, Snafu};
use std::iter::FromIterator;
use std::path::PathBuf;
use std::pin::Pin;
use std::process::Stdio;
use std::time::Duration;
use std::{env, io};
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::process::Child;
use tokio::process::Command;
use tokio::stream::StreamExt;
use tokio::time::timeout;

pub mod command;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Ip command error: {}", source))]
    CommandError { source: io::Error },

    #[snafu(display("Ip command failed, stdout: \"{}\", stderr: \"{}\"", stdout, stderr))]
    CommandFailedError { stdout: String, stderr: String },

    #[snafu(display("Command not found in PATH"))]
    CommandNotFoundError {},

    #[snafu(display("Command options error: {}", source))]
    CommandOptionsSerializationError { source: serde_command_opts::Error },

    #[snafu(display("Ip command timed out: {}", source))]
    CommandTimeoutError { source: tokio::time::Elapsed },

    #[snafu(display("Failed to deserialize json: {}", source))]
    JsonDeserializationError { source: serde_json::Error },

    #[snafu(display("Unable to spawn process: {}", source))]
    SpawnError { source: io::Error },
}

#[derive(Clone)]
pub struct IpCommand {
    command: PathBuf,
    timeout: Duration,
    namespace: Option<String>,
}

impl IpCommand {
    /// Create a new ip(8) command client.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            command: Self::path("ip").context(CommandNotFoundError {})?,
            timeout: Duration::from_millis(5_000),
            namespace: None,
        })
    }

    /// Return the current version of the ip(8) command.
    pub async fn version(&self) -> Result<String, Error> {
        self.command(&["-Version".into()], false).await
    }

    /// Create a new ip(8) command client for the specified network namespace.
    pub fn with_namespace(&self, namespace: &str) -> Self {
        let mut instance = self.clone();
        instance.namespace = Some(namespace.into());
        instance
    }

    /// Network device.
    pub fn link(&self) -> IpLinkCommand {
        IpLinkCommand::new(self)
    }

    /// Protocol (IP or IPv6) address on a device.
    pub fn address(&self) -> IpAddressCommand {
        IpAddressCommand::new(self)
    }

    /// Label configuration for protocol address selection.
    pub fn address_label(&self) -> IpAddressLabelCommand {
        IpAddressLabelCommand::new(self)
    }

    /// Routing table entry.
    pub fn route(&self) -> IpRouteCommand {
        IpRouteCommand::new(self)
    }

    /// Rule in routing policy database.
    pub fn rule(&self) -> IpRuleCommand {
        IpRuleCommand::new(self)
    }

    /// Manage ARP or NDISC cache entries.
    pub fn neighbor(&self) -> IpNeighborCommand {
        IpNeighborCommand::new(self)
    }

    /// Manage the neighbor cache's operation.
    pub fn neighbor_table(&self) -> IpNeighborTableCommand {
        IpNeighborTableCommand::new(self)
    }

    /// Tunnel over IP.
    pub fn tunnel(&self) -> IpTunnelCommand {
        IpTunnelCommand::new(self)
    }

    /// Manage TUN/TAP devices.
    pub fn tuntap(&self) -> IpTunTapCommand {
        IpTunTapCommand::new(self)
    }

    /// Multicast address.
    pub fn multicast_address(&self) -> IpMulticastAddressCommand {
        IpMulticastAddressCommand::new(self)
    }

    /// Multicast routing cache entry.
    pub fn multicast_route(&self) -> IpMulticastRouteCommand {
        IpMulticastRouteCommand::new(self)
    }

    /// Rule in multicast routing policy database.
    pub fn multicast_rule(&self) -> IpMulticastRuleCommand {
        IpMulticastRuleCommand::new(self)
    }

    /// Watch for netlink messages.
    pub fn monitor(&self) -> IpMonitorCommand {
        IpMonitorCommand::new(self)
    }

    /// Manage IPSec policies.
    pub fn transform(&self) -> IpTransformCommand {
        IpTransformCommand::new(self)
    }

    /// Manage network namespaces.
    pub fn netns(&self) -> IpNetNamespaceCommand {
        IpNetNamespaceCommand::new(self)
    }

    /// Tunnel ethernet over IP (L2TPv3).
    pub fn l2tp(&self) -> IpL2tpCommand {
        IpL2tpCommand::new(self)
    }

    /// Manage TCP Metrics.
    pub fn tcp_metrics(&self) -> IpTcpMetricsCommand {
        IpTcpMetricsCommand::new(self)
    }

    /// Manage tokenized interface identifiers.
    pub fn token(&self) -> IpTokenCommand {
        IpTokenCommand::new(self)
    }

    /// MACsec device configuration.
    pub fn macsec(&self) -> IpMACsecCommand {
        IpMACsecCommand::new(self)
    }

    pub(crate) async fn command(
        &self,
        args: &[String],
        combined_output: bool,
    ) -> Result<String, Error> {
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

    pub(crate) async fn command_with_streaming_output(
        &self,
        args: &[String],
        combined_output: bool,
    ) -> Result<ConsoleStream, Error> {
        // Disable console buffering using the stdbuf tool
        let mut combined_args: Vec<String> = vec![
            "-i0".into(),
            "-o0".into(),
            "-e0".into(),
            self.command.to_string_lossy().into(),
        ];
        combined_args.append(&mut self.concat_args(args)?);
        let process = Command::new(&Self::path("stdbuf").context(CommandNotFoundError {})?)
            .args(&combined_args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context(SpawnError {})?;
        ConsoleStream::new(process, combined_output)
    }

    fn concat_args(&self, args: &[String]) -> Result<Vec<String>, Error> {
        let mut combined_args: Vec<String> = vec!["-json".into()];
        if let Some(namespace) = &self.namespace {
            combined_args.push("-netns".into());
            combined_args.push(namespace.clone());
        }
        combined_args.append(&mut Vec::from_iter(args.iter().cloned()));
        Ok(combined_args)
    }

    fn path(name: &str) -> Option<PathBuf> {
        env::var_os("PATH").and_then(|paths| {
            env::split_paths(&paths)
                .filter_map(|dir| {
                    let full_path = dir.join(name);
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

/// A stream of strings corresponding to console lines.
pub struct ConsoleStream {
    _process: Child,
    inner: Pin<Box<dyn Stream<Item = tokio::io::Result<String>> + Send>>,
}

impl ConsoleStream {
    fn new(mut process: Child, combined_output: bool) -> Result<Self, Error> {
        let stdout = BufReader::new(process.stdout.take().unwrap()).lines();
        let inner: Pin<Box<dyn Stream<Item = tokio::io::Result<String>> + Send>> =
            if combined_output {
                let stderr = BufReader::new(process.stderr.take().unwrap()).lines();
                Box::pin(stdout.merge(stderr))
            } else {
                Box::pin(stdout)
            };
        Ok(Self {
            _process: process,
            inner,
        })
    }
}

impl Stream for ConsoleStream {
    type Item = tokio::io::Result<String>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(line) = ready!(self.inner.as_mut().poll_next(cx)) {
            Poll::Ready(Some(line))
        } else {
            Poll::Ready(None)
        }
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
