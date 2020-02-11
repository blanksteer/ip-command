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

use crate::*;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use serde_command_opts::{BooleanType, Serializer};
use snafu::ResultExt;

/// Virtual link add device configuration.
#[derive(Clone, Debug, Default, Serialize)]
pub struct LinkAddConfiguration {
    /// Name of the device.
    name: String,
    /// The physical device to operate on.
    #[serde(rename = "link")]
    device: Option<String>,
    /// Transmit queue length of the device.
    #[serde(rename = "txqueuelen")]
    transmit_queue_length: Option<u32>,
    /// Station address of the device.
    address: Option<String>,
    /// Link layer broadcast address.
    broadcast: Option<String>,
    /// Maximum transmission unit for the device.
    mtu: Option<u32>,
    /// Desired index of the device.
    index: Option<u32>,
    /// Number of transmit queues for device.
    #[serde(rename = "numtxqueues")]
    number_transmit_queues: Option<u32>,
    /// Number of receive queues for device.
    #[serde(rename = "numrxqueues")]
    number_receive_queues: Option<u32>,
    /// Maximum size of a Generic Segment Offload packet the device should accept.
    #[serde(rename = "gso_max_size")]
    gso_maximum_size: Option<u32>,
    /// Maximum number of a Generic Segment Offload segments the device should accept.
    #[serde(rename = "gso_max_segs")]
    gso_maximum_segments: Option<u32>,
    /// Type of the device.
    #[serde(rename = "type")]
    link_type: String,
}

#[derive(Clone, Debug)]
pub enum LinkDeviceOrGroup {
    Device(String),
    DeviceGroup(u32),
    None,
}

impl Default for LinkDeviceOrGroup {
    fn default() -> Self {
        Self::None
    }
}

impl Serialize for LinkDeviceOrGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Device(device) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("dev")?;
                seq.serialize_element(device)?;
                seq.end()
            }
            Self::DeviceGroup(group) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("group")?;
                seq.serialize_element(group)?;
                seq.end()
            }
            _ => unimplemented!(),
        }
    }
}

/// Virtual link delete device configuration.
#[derive(Clone, Debug, Default, Serialize)]
pub struct LinkDeleteConfiguration {
    device: LinkDeviceOrGroup,
    /// Type of the device.
    #[serde(rename = "type")]
    link_type: String,
}

#[derive(Clone, Debug)]
pub enum LinkStatus {
    Up,
    Down,
}

impl Serialize for LinkStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(1))?;
        match self {
            Self::Up => {
                seq.serialize_element("up")?;
            }
            Self::Down => {
                seq.serialize_element("down")?;
            }
        }
        seq.end()
    }
}

#[derive(Clone, Debug)]
pub enum MasterSetConfiguration {
    /// Set master device of the device.
    Enslaved(String),
    /// Unset master device of the device.
    Release,
}

impl Serialize for MasterSetConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Enslaved(device) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("dev")?;
                seq.serialize_element(device)?;
                seq.end()
            }
            Self::Release => {
                let mut seq = serializer.serialize_seq(Some(1))?;
                seq.serialize_element("nomaster")?;
                seq.end()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExpressDataPathVariant {
    /// Kernel gets to choose the best available mode.
    Default,
    /// Use the slow generic fallback mode.
    Generic,
    /// Use a fast driver based mode, if not available then error.
    Driver,
    /// Hardware offload mode.
    Offload,
}

impl Default for ExpressDataPathVariant {
    fn default() -> Self {
        Self::Default
    }
}

impl ToString for ExpressDataPathVariant {
    fn to_string(&self) -> String {
        match self {
            ExpressDataPathVariant::Default => "xdp".into(),
            ExpressDataPathVariant::Generic => "xdpgeneric".into(),
            ExpressDataPathVariant::Driver => "xdpdrv".into(),
            ExpressDataPathVariant::Offload => "xdpoffload".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressDataPathConfiguration {
    Off,
    Pinned {
        variant: ExpressDataPathVariant,
        path: String,
        verbose: bool,
    },
    Object {
        variant: ExpressDataPathVariant,
        path: String,
        section_name: Option<String>,
        verbose: bool,
    },
}

impl Default for ExpressDataPathConfiguration {
    fn default() -> Self {
        Self::Off
    }
}

impl Serialize for ExpressDataPathConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ExpressDataPathConfiguration::Object {
                variant,
                path,
                section_name,
                verbose,
            } => {
                let elements = if *verbose && section_name.is_some() {
                    6
                } else if *verbose {
                    4
                } else {
                    3
                };
                let mut seq = serializer.serialize_seq(Some(elements))?;
                seq.serialize_element(&variant.to_string())?;
                seq.serialize_element("object")?;
                seq.serialize_element(&path)?;
                if let Some(section_name) = section_name {
                    seq.serialize_element("section")?;
                    seq.serialize_element(&section_name)?;
                }
                if *verbose {
                    seq.serialize_element("verbose")?;
                }
                seq.end()
            }
            ExpressDataPathConfiguration::Pinned {
                variant,
                path,
                verbose,
            } => {
                let mut seq = serializer.serialize_seq(Some(if *verbose { 4 } else { 3 }))?;
                seq.serialize_element(&variant.to_string())?;
                seq.serialize_element("pinned")?;
                seq.serialize_element(&path)?;
                if *verbose {
                    seq.serialize_element("verbose")?;
                }
                seq.end()
            }
            ExpressDataPathConfiguration::Off => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("xdp")?;
                seq.serialize_element("off")?;
                seq.end()
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LinkSetConfiguration {
    /// The device or group to operate on.
    device: LinkDeviceOrGroup,
    /// Change the state of the device.
    state: Option<LinkStatus>,
    /// Enable or disable the use of the Address Resolution Protocol.
    arp: Option<bool>,
    /// Enable or disable support for multicast packets.
    multicast: Option<bool>,
    /// Enable or disable the reception of all hardware multicast packets.
    #[serde(rename = "allmulticast")]
    all_multicast: Option<bool>,
    /// Enable or disable promiscuous listening mode.
    #[serde(rename = "promisc")]
    promiscuous: Option<bool>,
    /// Indicate that a protocol error has been detected.
    #[serde(rename = "protodown")]
    protocol_down: Option<bool>,
    /// Enable or disable the use of trailer encapsulations.
    trailers: Option<bool>,
    /// Transmit queue length of the device.
    #[serde(rename = "txqueuelen")]
    transmit_queue_length: Option<u32>,
    /// Change the name of the device.
    #[serde(rename = "name")]
    new_name: Option<String>,
    /// Station address of the device.
    address: Option<String>,
    /// Link layer broadcast address.
    broadcast: Option<String>,
    /// Maximum transmission unit for the device.
    mtu: Option<u32>,
    /// Move the device to the supplied network namespace or pid.
    #[serde(rename = "netns")]
    namespace: Option<String>,
    /// Set peer netnsid for a cross-netns interface.
    #[serde(rename = "link-netnsid")]
    link_network_namespace_id: Option<u32>,
    /// Set / unset the master device of the device.
    master: Option<MasterSetConfiguration>,
    /// Enslave to virtual routing and forwarding master.
    #[serde(rename = "vrf")]
    vrf_master: Option<String>,
    /// IPv6 address generation mode.
    #[serde(rename = "addrgenmode")]
    address_generation_mode: Option<String>,
    /// Set (or unset) a BPF program to run on every packet at driver level.
    express_data_path: Option<ExpressDataPathConfiguration>,
    /// Type of the device.
    #[serde(rename = "type")]
    link_type: Option<String>,
    // Any type specific arguments: currently not supported.
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct LinkShowConfiguration {
    /// The network device to show.
    device: LinkDeviceOrGroup,
    /// Only display running interfaces.
    #[serde(skip_serializing)]
    state: Option<LinkStatus>,
    /// Master device which enslaves devices to show.
    master: Option<String>,
    /// The VRF which enslaves devices to show.
    #[serde(rename = "vrf")]
    virtual_function_device: Option<String>,
    /// The type of devices to show.
    #[serde(rename = "type")]
    link_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExpressDataPathProgram {
    pub id: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExpressDataPath {
    pub mode: u32,
    #[serde(rename = "prog")]
    pub program: Option<ExpressDataPathProgram>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Link {
    #[serde(rename = "ifindex")]
    pub interface_index: u32,
    #[serde(rename = "ifname")]
    pub name: String,
    #[serde(rename = "flags")]
    pub flags: Vec<String>,
    pub mtu: u32,
    #[serde(rename = "qdisc")]
    pub queueing_discipline: String,
    #[serde(rename = "operstate")]
    pub state: String,
    #[serde(rename = "linkmode")]
    pub link_mode: Option<String>,
    #[serde(rename = "group")]
    pub group: Option<String>,
    #[serde(rename = "txqlen")]
    pub transmit_queue_length: Option<u32>,
    pub link_type: Option<String>,
    pub address: Option<String>,
    pub broadcast: Option<String>,
    #[serde(rename = "xdp")]
    pub express_data_path: Option<ExpressDataPath>,
}

#[derive(Clone)]
pub struct IpLinkCommand<'l> {
    ip_command: &'l IpCommand,
}

impl<'l> IpLinkCommand<'l> {
    pub fn new(ip_command: &'l IpCommand) -> Self {
        Self { ip_command }
    }

    /// Add virtual link.
    pub async fn add(&self, configuration: LinkAddConfiguration) -> Result<(), Error> {
        let mut args: Vec<String> = vec!["link".into(), "add".into()];
        args.append(
            &mut Serializer::new(BooleanType::OnOff)
                .into_args(&configuration)
                .context(CommandOptionsSerializationError {})?,
        );
        self.ip_command.command(&args, false).await.map(|_| ())
    }

    /// Delete virtual link.
    pub async fn delete(&self, configuration: LinkDeleteConfiguration) -> Result<(), Error> {
        let mut args: Vec<String> = vec!["link".into(), "delete".into()];
        args.append(
            &mut Serializer::new(BooleanType::OnOff)
                .into_args(&configuration)
                .context(CommandOptionsSerializationError {})?,
        );
        self.ip_command.command(&args, false).await.map(|_| ())
    }

    /// Change device attributes.
    pub async fn set(&self, configuration: LinkSetConfiguration) -> Result<(), Error> {
        let mut args: Vec<String> = vec!["link".into(), "set".into()];
        args.append(
            &mut Serializer::new(BooleanType::OnOff)
                .into_args(&configuration)
                .context(CommandOptionsSerializationError {})?,
        );
        self.ip_command.command(&args, false).await.map(|_| ())
    }

    /// Display device attributes.
    pub async fn show(
        &self,
        configuration: Option<LinkShowConfiguration>,
    ) -> Result<Vec<Link>, Error> {
        let mut args: Vec<String> = vec!["link".into(), "show".into()];
        if let Some(configuration) = configuration {
            args.append(
                &mut Serializer::new(BooleanType::OnOff)
                    .into_args(&configuration)
                    .context(CommandOptionsSerializationError {})?,
            );
        }
        let output = self.ip_command.command(&args, false).await?;
        Ok(serde_json::from_str(&output).context(JsonDeserializationError {})?)
    }

    /// Display extended statistics.
    pub async fn xstats(&self) -> Result<(), Error> {
        // No support for JSON formatting combined with loosely defined fields means this
        // feature will remain unsupported.
        unimplemented!()
    }

    /// Display address-family specific statistics.
    pub async fn afstats(&self) -> Result<(), Error> {
        // Non functional for at least the vast majority of interface types on debian stable
        // until it can be proved to be functional this feature will remain unsupported.
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add() {
        let link_name = "test0";

        let configuration = LinkAddConfiguration {
            name: link_name.into(),
            device: None,
            link_type: "dummy".into(),
            transmit_queue_length: Some(1u32),
            address: Some("02:00:00:00:01:00".into()),
            broadcast: Some("FF:FF:FF:FF:FF:FF".into()),
            mtu: Some(1400u32),
            index: Some(100u32),
            number_transmit_queues: Some(1u32),
            number_receive_queues: Some(1u32),
            gso_maximum_size: Some(65536u32),
            gso_maximum_segments: Some(10u32),
        };

        let client = IpCommand::new().unwrap();
        client.link().add(configuration).await.unwrap();

        let links = client.link().show(None).await.unwrap();
        let link = links
            .into_iter()
            .find(|link| link.name.eq_ignore_ascii_case(link_name))
            .unwrap();
        client
            .link()
            .delete(LinkDeleteConfiguration {
                device: LinkDeviceOrGroup::Device(link_name.into()),
                link_type: "dummy".into(),
            })
            .await
            .unwrap();

        assert_eq!(link.name, link_name.to_string());
        assert_eq!(link.transmit_queue_length, Some(1u32));
        assert_eq!(link.address, Some("02:00:00:00:01:00".into()));
        assert_eq!(link.broadcast, Some("ff:ff:ff:ff:ff:ff".into()));
        assert_eq!(link.mtu, 1400u32);
        assert_eq!(link.interface_index, 100u32);
        assert_eq!(link.transmit_queue_length, Some(1u32));
    }

    #[tokio::test]
    async fn test_show_and_delete() {
        let link_name = "test1";
        let client = IpCommand::new().unwrap();

        client
            .link()
            .add(LinkAddConfiguration {
                name: link_name.into(),
                link_type: "dummy".into(),
                ..Default::default()
            })
            .await
            .unwrap();

        let link = client
            .link()
            .show(Some(LinkShowConfiguration {
                device: LinkDeviceOrGroup::Device(link_name.into()),
                ..Default::default()
            }))
            .await
            .unwrap();

        assert!(!link.is_empty());

        client
            .link()
            .delete(LinkDeleteConfiguration {
                device: LinkDeviceOrGroup::Device(link_name.into()),
                link_type: "dummy".into(),
            })
            .await
            .unwrap();

        // Interface should no longer be found
        let link = client
            .link()
            .show(None)
            .await
            .unwrap()
            .into_iter()
            .find(|link| link.name.eq(link_name));

        assert!(link.is_none());
    }

    #[tokio::test]
    async fn test_set_dummy() {
        let link_name = "test2";
        let test_namespace = "ip-command-test-link-set-dummy";

        let client = IpCommand::new().unwrap();

        client
            .link()
            .add(LinkAddConfiguration {
                name: link_name.into(),
                link_type: "dummy".into(),
                ..Default::default()
            })
            .await
            .unwrap();

        let configuration = LinkSetConfiguration {
            device: LinkDeviceOrGroup::Device(link_name.into()),
            state: Some(LinkStatus::Up),
            link_type: Some("dummy".into()),
            arp: Some(false),
            multicast: Some(true),
            all_multicast: Some(true),
            promiscuous: Some(false),
            protocol_down: None,
            trailers: Some(false),
            transmit_queue_length: Some(1u32),
            new_name: Some("dummy1".into()),
            address: Some("02:00:00:00:01:01".into()),
            broadcast: Some("FF:FF:FF:FF:FF:FF".into()),
            mtu: Some(1400),
            namespace: Some(test_namespace.into()),
            link_network_namespace_id: Some(101u32),
            express_data_path: None,
            master: None,
            vrf_master: None,
            address_generation_mode: Some("eui64".into()),
        };

        client.netns().add(test_namespace).await.unwrap();
        client.link().set(configuration).await.unwrap();

        // We rename the interface
        let link_name = "dummy1";

        let link = client
            .with_namespace(test_namespace)
            .link()
            .show(Some(LinkShowConfiguration {
                device: LinkDeviceOrGroup::Device(link_name.into()),
                ..Default::default()
            }))
            .await
            .unwrap();

        client
            .with_namespace(test_namespace)
            .link()
            .delete(LinkDeleteConfiguration {
                device: LinkDeviceOrGroup::Device(link_name.into()),
                link_type: "dummy".into(),
            })
            .await
            .unwrap();

        client.netns().delete(test_namespace).await.unwrap();

        assert!(link[0].flags.contains(&"UP".to_string()));
        assert!(link[0].flags.contains(&"NOARP".to_string()));
        assert!(link[0].flags.contains(&"MULTICAST".to_string()));
        assert!(link[0].flags.contains(&"ALLMULTI".to_string()));
        assert!(link[0].flags.contains(&"NOTRAILERS".to_string()));
        assert_eq!(link[0].transmit_queue_length, Some(1u32));
        assert_eq!(link[0].address, Some("02:00:00:00:01:01".into()));
        assert_eq!(link[0].broadcast, Some("ff:ff:ff:ff:ff:ff".into()));
        assert_eq!(link[0].mtu, 1400);
    }

    #[tokio::test]
    async fn test_set_xdp() {
        let link_name = "test3";

        let client = IpCommand::new().unwrap();
        client
            .link()
            .add(LinkAddConfiguration {
                name: link_name.into(),
                link_type: "dummy".into(),
                ..Default::default()
            })
            .await
            .unwrap();

        let configuration = LinkSetConfiguration {
            device: LinkDeviceOrGroup::Device(link_name.into()),
            state: Some(LinkStatus::Up),
            express_data_path: Some(ExpressDataPathConfiguration::Object {
                variant: ExpressDataPathVariant::Default,
                path: "src/command/test_fixture/xdp_test.o".into(),
                section_name: Some("xdp".into()),
                verbose: false
            }),
            ..Default::default()
        };

        client.link().set(configuration).await.unwrap();

        let link = client.link()
            .show(Some(LinkShowConfiguration {
                device: LinkDeviceOrGroup::Device(link_name.into()),
                ..Default::default()
            }))
            .await
            .unwrap();

        client.link()
            .delete(LinkDeleteConfiguration {
                device: LinkDeviceOrGroup::Device(link_name.into()),
                link_type: "dummy".into(),
            })
            .await
            .unwrap();

        assert_eq!(link.len(), 1);
        assert!(link[0].express_data_path.is_some());
    }
}
