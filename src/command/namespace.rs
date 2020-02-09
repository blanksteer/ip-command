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

use crate::*;
use serde::Deserialize;
use snafu::ResultExt;

#[derive(Debug, Clone, Deserialize)]
pub struct Namespace {
    name: String,
    #[serde(alias = "id", alias = "nsid")]
    id: Option<u32>,
}

#[derive(Clone)]
pub struct IpNetNamespaceCommand<'l> {
    ip_command: &'l IpCommand,
}

impl<'l> IpNetNamespaceCommand<'l> {
    pub fn new(ip_command: &'l IpCommand) -> Self {
        Self { ip_command }
    }

    /// Show all of the named network namespaces.
    pub async fn list(&self) -> Result<Vec<Namespace>, Error> {
        let output = self
            .ip_command
            .command(&["netns".into(), "list".into()], false)
            .await?;
        Ok(serde_json::from_str(&output).context(JsonDeserializationError {})?)
    }

    /// Create a new named network namespace.
    pub async fn add(&self, network_namespace_name: &str) -> Result<(), Error> {
        self.ip_command
            .command(
                &["netns".into(), "add".into(), network_namespace_name.into()],
                false,
            )
            .await
            .map(|_| ())
    }

    /// Delete the name of a network.
    pub async fn delete(&self, network_namespace_name: &str) -> Result<(), Error> {
        self.ip_command
            .command(
                &["netns".into(), "del".into(), network_namespace_name.into()],
                false,
            )
            .await
            .map(|_| ())
    }

    /// Assign an id to a peer network namespace.
    pub async fn set(
        &self,
        network_namespace_name: &str,
        network_namespace_id: Option<u32>,
    ) -> Result<(), Error> {
        let network_namespace_id = network_namespace_id
            .map(|id| format!("{}", id))
            .unwrap_or_else(|| "auto".into());
        self.ip_command
            .command(
                &[
                    "netns".into(),
                    "set".into(),
                    network_namespace_name.into(),
                    network_namespace_id,
                ],
                false,
            )
            .await
            .map(|_| ())
    }

    /// Report network namespaces names for process.
    pub async fn identify(&self, process_id: u32) -> Result<String, Error> {
        self.ip_command
            .command(
                &["netns".into(), "identify".into(), format!("{}", process_id)],
                false,
            )
            .await
            .map(|result| result.trim().into())
    }

    /// Report processes in the named network namespace.
    pub async fn pids(&self, network_namespace_name: &str) -> Result<Vec<u32>, Error> {
        let output = self
            .ip_command
            .command(
                &["netns".into(), "pids".into(), network_namespace_name.into()],
                false,
            )
            .await?;
        Ok(Vec::from_iter(
            output
                .split_whitespace()
                .map(|id| id.parse::<u32>().unwrap()),
        ))
    }

    /// Run command in the named network namespace.
    pub async fn exec(
        &self,
        network_namespace_name: &str,
        command_and_args: &[String],
    ) -> Result<ConsoleStream, Error> {
        let mut args: Vec<String> =
            vec!["netns".into(), "exec".into(), network_namespace_name.into()];
        args.append(&mut Vec::from(command_and_args));
        self.ip_command
            .command_with_streaming_output(&args, false)
            .await
    }

    /// Report as network namespace names are added and deleted.
    pub async fn monitor(&self) -> Result<ConsoleStream, Error> {
        self.ip_command
            .command_with_streaming_output(&["netns".into(), "monitor".into()], false)
            .await
    }

    /// List network namespace ids.
    pub async fn list_id(
        &self,
        _target_network_namespace_id: Option<u32>,
        _network_namespace_id: Option<u32>,
    ) -> Result<Vec<Namespace>, Error> {
        // TODO: enable once the target-nsid feature is in stable linux distros.
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::channel::oneshot::channel;
    use futures::StreamExt;
    use tokio::time::delay_for;

    #[tokio::test]
    async fn test_add_list_and_delete() {
        let test_namespace = "ip-command-test-add-namespace";
        let client = IpCommand::new().unwrap();

        client.netns().add(test_namespace).await.unwrap();

        let namespaces = client.netns().list().await.unwrap();
        assert!(namespaces
            .iter()
            .any(|namespace| namespace.name.eq(test_namespace)));

        client.netns().delete(test_namespace).await.unwrap();

        let namespaces = client.netns().list().await.unwrap();
        assert!(namespaces
            .iter()
            .find(|namespace| namespace.name.eq(test_namespace))
            .is_none());
    }

    #[tokio::test]
    async fn test_set() {
        let test_namespace = "ip-command-test-set-namespace";
        let client = IpCommand::new().unwrap();

        client.netns().add(test_namespace).await.unwrap();
        client.netns().set(test_namespace, Some(500)).await.unwrap();

        let namespaces = client.netns().list().await.unwrap();
        let namespace = namespaces
            .iter()
            .find(|namespace| namespace.name.eq(test_namespace))
            .unwrap();

        assert_eq!(namespace.id.unwrap(), 500);

        client.netns().delete(test_namespace).await.unwrap();
    }

    #[tokio::test]
    async fn test_exec_and_identify() {
        let test_namespace = "ip-command-test-exec-and-identify-namespace";

        let (pid_sender, pid_receiver) = channel::<u32>();
        let manifest_path = env::var("CARGO_MANIFEST_DIR").unwrap();
        let test_binary = manifest_path + "/target/debug/namespaced_process";

        let client = IpCommand::new().unwrap();
        client.netns().add(test_namespace).await.unwrap();

        let exec_client = client.clone();
        tokio::spawn(async move {
            let mut console_stream = exec_client
                .netns()
                .exec(test_namespace, &[test_binary, "5".into()])
                .await
                .unwrap();
            if let Some(Ok(next_line)) = console_stream.next().await {
                pid_sender.send(next_line.parse().unwrap()).unwrap();
            }
        });

        let pid = pid_receiver.await.unwrap();
        let process_namespace = client.netns().identify(pid).await.unwrap();
        assert_eq!(&process_namespace, test_namespace);

        client.netns().delete(test_namespace).await.unwrap();
    }

    #[tokio::test]
    async fn test_exec_and_pids() {
        let test_namespace = "ip-command-test-exec-and-pids-namespace";

        let (pid_sender, pid_receiver) = channel::<u32>();
        let manifest_path = env::var("CARGO_MANIFEST_DIR").unwrap();
        let test_binary = manifest_path + "/target/debug/namespaced_process";

        let client = IpCommand::new().unwrap();
        client.netns().add(test_namespace).await.unwrap();

        let exec_client = client.clone();
        tokio::spawn(async move {
            let mut console_stream = exec_client
                .netns()
                .exec(test_namespace, &[test_binary, "5".into()])
                .await
                .unwrap();
            if let Some(Ok(next_line)) = console_stream.next().await {
                pid_sender.send(next_line.parse().unwrap()).unwrap();
            }
        });

        let pid = pid_receiver.await.unwrap();

        let pids = client.netns().pids(test_namespace).await.unwrap();
        assert_eq!(pids[0], pid);

        client.netns().delete(test_namespace).await.unwrap();
    }

    #[tokio::test]
    async fn test_monitor() {
        let test_namespace = "ip-command-test-monitor-namespace";
        let (result_sender, result_receiver) = channel::<String>();

        let client = IpCommand::new().unwrap();
        let monitor_client = client.clone();

        tokio::spawn(async move {
            let mut console_stream = monitor_client.netns().monitor().await.unwrap();
            while let Some(Ok(next_line)) = console_stream.next().await {
                if next_line.contains("ip-command-test-monitor-namespace") {
                    result_sender.send(next_line).unwrap();
                    break;
                }
            }
        });

        delay_for(Duration::from_millis(500)).await;

        client.netns().add(test_namespace).await.unwrap();

        assert_eq!(
            &result_receiver.await.unwrap(),
            "add ip-command-test-monitor-namespace"
        );

        client.netns().delete(test_namespace).await.unwrap();
    }
}
