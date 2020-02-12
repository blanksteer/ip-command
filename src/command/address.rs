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

use crate::command::link::LinkStatus;
use crate::*;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use serde_command_opts::{BooleanType, Serializer};
use snafu::ResultExt;

/// Add protocol address configuration flags.
#[derive(Clone, Debug)]
pub enum AddressAddConfigurationFlag {
    None,
    /// Designates this address the "home address" as defined in RFC 6275 (IPv6 only).
    HomeAddress,
    /// Make the kernel manage temporary addresses created from this one as template on behalf of RFC 3041 (IPv6 only).
    KernelManagedTemporaryAddress,
    /// Do not perform Duplicate Address Detection RFC 4862 (IPv6 only).
    NoDuplicateAddressDetection,
    /// Do not automatically create a route for the network prefix of the added address.
    NoPrefixRoute,
    /// Automatically join multicast groups.
    JoinMulticastGroups,
}

impl Default for AddressAddConfigurationFlag {
    fn default() -> Self {
        Self::None
    }
}

impl ToString for AddressAddConfigurationFlag {
    fn to_string(&self) -> String {
        match self {
            Self::HomeAddress => "home".into(),
            Self::KernelManagedTemporaryAddress => "mngtmpaddr".into(),
            Self::NoDuplicateAddressDetection => "nodad".into(),
            Self::NoPrefixRoute => "noprefixroute".into(),
            Self::JoinMulticastGroups => "autojoin".into(),
            _ => unimplemented!(),
        }
    }
}

impl Serialize for AddressAddConfigurationFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Self::None = self {
            serializer.serialize_none()
        } else {
            let mut seq = serializer.serialize_seq(Some(1))?;
            seq.serialize_element(&self.to_string())?;
            seq.end()
        }
    }
}

/// Add protocol address configuration.
#[derive(Clone, Debug, Default, Serialize)]
pub struct AddressAddConfiguration {
    /// The address of the interface.
    pub local: String,
    /// The address of the remote endpoint for pointopoint interfaces.
    pub peer: Option<String>,
    /// The broadcast address on the interface.
    pub broadcast: Option<String>,
    /// The anycast address.
    #[serde(rename = "anycast")]
    pub any_cast: Option<String>,
    /// Label for tagging the address.
    pub label: Option<String>,
    /// The scope of the area where this address is valid.
    pub scope: Option<String>,
    /// The name of the device to add the address to.
    #[serde(rename = "dev")]
    pub device: String,
    /// The valid lifetime of this address (seconds or "forever").
    #[serde(rename = "valid_lft")]
    pub valid_lifetime: Option<String>,
    /// The preferred lifetime of this address (seconds or "forever").
    #[serde(rename = "preferred_lft")]
    pub preferred_lifetime: Option<String>,
    /// Optional configuration flags.
    pub flags: Option<Vec<AddressAddConfigurationFlag>>,
}

/// List protocol addresses configuration flags.
#[derive(Clone, Debug)]
pub enum AddressConfigurationFlag {
    None,
    /// Only list addresses installed due to stateless address configuration (IPv6 only).
    Dynamic,
    /// Only list permanent (not dynamic) addresses (IPv6 only).
    Permanent,
    /// Only list addresses which have not yet passed duplicate address detection (IPv6 only).
    Tentative,
    /// Only list addresses which are not in the process of duplicate address detection currently (IPv6 only).
    NotTentative,
    /// Only list deprecated addresses (IPv6 only).
    Deprecated,
    /// Only list addresses not being deprecated (IPv6 only).
    NotDeprecated,
    /// Only list addresses which have failed duplicate address detection (IPv6 only).
    DuplicateAddressDetectionFailed,
    /// Only list addresses which have not failed duplicate address detection (IPv6 only).
    NotDuplicateAddressDetectionFailed,
    /// List only primary addresses, in IPv6 exclude temporary ones.
    Primary,
    /// List secondary IPv4 addresses only.
    Secondary,
    /// List temporary IPv6 addresses only.
    Temporary,
}

impl Default for AddressConfigurationFlag {
    fn default() -> Self {
        Self::None
    }
}

impl ToString for AddressConfigurationFlag {
    fn to_string(&self) -> String {
        match self {
            Self::Dynamic => "dynamic".into(),
            Self::Permanent => "permanent".into(),
            Self::Tentative => "tentative".into(),
            Self::NotTentative => "-tentative".into(),
            Self::Deprecated => "deprecated".into(),
            Self::NotDeprecated => "-deprecated".into(),
            Self::DuplicateAddressDetectionFailed => "dadfailed".into(),
            Self::NotDuplicateAddressDetectionFailed => "-dadfailed".into(),
            Self::Primary => "primary".into(),
            Self::Secondary => "secondary".into(),
            Self::Temporary => "temporary".into(),
            _ => unimplemented!(),
        }
    }
}

impl Serialize for AddressConfigurationFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Self::None = self {
            serializer.serialize_none()
        } else {
            let mut seq = serializer.serialize_seq(Some(1))?;
            seq.serialize_element(&self.to_string())?;
            seq.end()
        }
    }
}

/// List protocol addresses configuration.
#[derive(Clone, Debug, Default, Serialize)]
pub struct AddressShowConfiguration {
    /// The name of the device.
    #[serde(rename = "dev")]
    pub device: String,
    /// Only list addresses with this scope.
    pub scope: Option<String>,
    /// Only list addresses matching this prefix.
    pub to: Option<String>,
    /// Only list addresses with labels matching the pattern.
    pub label: Option<String>,
    /// Only list interfaces enslaved to this master device.
    pub master: Option<String>,
    /// Only list interfaces enslaved to this vrf.
    #[serde(rename = "vrf")]
    pub vrf_master: Option<String>,
    /// Only list interfaces of the given type.
    #[serde(rename = "type")]
    pub link_type: Option<String>,
    /// Only list running interfaces.
    pub state: Option<LinkStatus>,
    /// Optional configuration flags.
    pub flags: Option<Vec<AddressConfigurationFlag>>,
}

/// Delete protocol address configuration.
#[derive(Clone, Debug, Default, Serialize)]
pub struct AddressDeleteConfiguration {
    /// The address of the interface.
    pub local: String,
    /// The address of the remote endpoint for pointopoint interfaces.
    pub peer: Option<String>,
    /// The broadcast address on the interface.
    pub broadcast: Option<String>,
    /// The anycast address.
    #[serde(rename = "anycast")]
    pub any_cast: Option<String>,
    /// Label for tagging the address.
    pub label: Option<String>,
    /// The scope of the area where this address is valid.
    pub scope: Option<String>,
    /// The name of the device.
    #[serde(rename = "dev")]
    pub device: String,
    /// Optional configuration flags (only KernelManagedTemporaryAddress supported).
    pub flags: Option<Vec<AddressAddConfigurationFlag>>,
}

/// Flush/save protocol address configuration.
#[derive(Clone, Debug, Default, Serialize)]
pub struct AddressFlushOrSaveConfiguration {
    /// The name of the device.
    #[serde(rename = "dev")]
    pub device: Option<String>,
    /// Only match addresses with this scope.
    pub scope: Option<String>,
    /// Only match addresses with this prefix route priority.
    pub metric: Option<u32>,
    /// Only match addresses matching this prefix.
    pub to: Option<String>,
    /// Optional configuration flags.
    pub flags: Option<Vec<AddressConfigurationFlag>>,
    /// Only match addresses with labels matching the pattern.
    pub label: Option<String>,
    /// Only match running interfaces.
    pub state: Option<LinkStatus>,
}

pub type AddressFlushConfiguration = AddressFlushOrSaveConfiguration;
pub type AddressSaveConfiguration = AddressFlushOrSaveConfiguration;

/// The returned address information structure.
#[derive(Debug, Clone, Deserialize)]
pub struct AddressInfo {
    pub family: Option<String>,
    pub local: Option<String>,
    #[serde(rename = "prefixlen")]
    pub prefix_length: Option<u32>,
    pub broadcast: Option<String>,
    pub anycast: Option<String>,
    pub scope: Option<String>,
    pub dynamic: Option<bool>,
    #[serde(rename = "noprefixroute")]
    pub no_prefix_route: Option<bool>,
    pub label: Option<String>,
    pub valid_life_time: Option<u32>,
    pub preferred_life_time: Option<u32>,
}

/// The returned address structure.
#[derive(Debug, Clone, Deserialize)]
pub struct Address {
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
    pub group: Option<String>,
    #[serde(rename = "txqlen")]
    pub transmit_queue_length: Option<u32>,
    pub link_type: Option<String>,
    pub address: Option<String>,
    pub broadcast: Option<String>,
    #[serde(rename = "addr_info")]
    pub address_info: Option<Vec<AddressInfo>>,
}

#[derive(Clone)]
pub struct IpAddressCommand<'l> {
    ip_command: &'l IpCommand,
}

impl<'l> IpAddressCommand<'l> {
    pub fn new(ip_command: &'l IpCommand) -> Self {
        Self { ip_command }
    }

    /// Add new protocol address.
    pub async fn add(&self, configuration: AddressAddConfiguration) -> Result<(), Error> {
        let mut args: Vec<String> = vec!["address".into(), "add".into()];
        args.append(
            &mut Serializer::new(BooleanType::OnOff)
                .into_args(&configuration)
                .context(CommandOptionsSerializationError {})?,
        );
        self.ip_command
            .command(&args, false, None)
            .await
            .map(|_| ())
    }

    /// Modify the flags on an existing protocol address.
    pub async fn change(&self, configuration: AddressAddConfiguration) -> Result<(), Error> {
        let mut args: Vec<String> = vec!["address".into(), "change".into()];
        args.append(
            &mut Serializer::new(BooleanType::OnOff)
                .into_args(&configuration)
                .context(CommandOptionsSerializationError {})?,
        );
        self.ip_command
            .command(&args, false, None)
            .await
            .map(|_| ())
    }

    /// Add new or modify existing protocol address.
    pub async fn replace(&self, configuration: AddressAddConfiguration) -> Result<(), Error> {
        let mut args: Vec<String> = vec!["address".into(), "replace".into()];
        args.append(
            &mut Serializer::new(BooleanType::OnOff)
                .into_args(&configuration)
                .context(CommandOptionsSerializationError {})?,
        );
        self.ip_command
            .command(&args, false, None)
            .await
            .map(|_| ())
    }

    /// Delete protocol address.
    pub async fn delete(&self, configuration: AddressDeleteConfiguration) -> Result<(), Error> {
        let mut args: Vec<String> = vec!["address".into(), "del".into()];
        args.append(
            &mut Serializer::new(BooleanType::OnOff)
                .into_args(&configuration)
                .context(CommandOptionsSerializationError {})?,
        );
        self.ip_command
            .command(&args, false, None)
            .await
            .map(|_| ())
    }

    /// Look at protocol addresses.
    pub async fn show(
        &self,
        configuration: Option<AddressShowConfiguration>,
    ) -> Result<Vec<Address>, Error> {
        let mut args: Vec<String> = vec!["address".into(), "show".into()];
        if let Some(configuration) = configuration {
            args.append(
                &mut Serializer::new(BooleanType::OnOff)
                    .into_args(&configuration)
                    .context(CommandOptionsSerializationError {})?,
            );
        }
        let mut output = self.ip_command.command(&args, false, None).await?;
        // Strip out invalid junk the iproute2 json serializer produces.
        output = output.replace("{},", "");
        output = output.replace(",{}", "");
        Ok(serde_json::from_str(&output).context(JsonDeserializationError {})?)
    }

    /// Flush protocol addresses.
    pub async fn flush(
        &self,
        configuration: Option<AddressFlushConfiguration>,
    ) -> Result<(), Error> {
        let mut args: Vec<String> = vec!["address".into(), "flush".into()];
        if let Some(configuration) = configuration {
            args.append(
                &mut Serializer::new(BooleanType::OnOff)
                    .into_args(&configuration)
                    .context(CommandOptionsSerializationError {})?,
            );
        }
        self.ip_command
            .command(&args, false, None)
            .await
            .map(|_| ())
    }

    /// Save the protocol address raw configuration.
    pub async fn save(
        &self,
        configuration: Option<AddressSaveConfiguration>,
    ) -> Result<Vec<u8>, Error> {
        let mut args: Vec<String> = vec!["address".into(), "save".into()];
        if let Some(configuration) = configuration {
            args.append(
                &mut Serializer::new(BooleanType::OnOff)
                    .into_args(&configuration)
                    .context(CommandOptionsSerializationError {})?,
            );
        }
        self.ip_command
            .command_with_raw_output(&args, false, None)
            .await
    }

    /// Restore the protocol address from a raw configuration.
    pub async fn restore(&self, netlink_configuration: Vec<u8>) -> Result<(), Error> {
        let args: Vec<String> = vec!["address".into(), "restore".into()];
        self.ip_command
            .command(&args, false, Some(netlink_configuration))
            .await
            .map(|_| ())
    }

    /// Convert the raw netlink configuration into a human readable form or json.
    pub async fn show_dump(&self) -> Result<(), Error> {
        // Left out as its utility is somewhat limited for the vast majority of automated usecases.
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::link::{LinkAddConfiguration, LinkDeleteConfiguration, LinkDeviceOrGroup};

    #[tokio::test]
    async fn test_add_and_show() {
        let link_name = "test_addr0";
        let address = "172.80.0.1";
        let broadcast_address = "172.80.0.255";
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

        client
            .address()
            .add(AddressAddConfiguration {
                local: address.into(),
                broadcast: Some(broadcast_address.into()),
                device: link_name.into(),
                ..Default::default()
            })
            .await
            .unwrap();

        let addresses = client
            .address()
            .show(Some(AddressShowConfiguration {
                device: link_name.into(),
                ..Default::default()
            }))
            .await
            .unwrap();

        client
            .link()
            .delete(LinkDeleteConfiguration {
                device: LinkDeviceOrGroup::Device(link_name.into()),
                link_type: "dummy".into(),
            })
            .await
            .unwrap();

        assert_eq!(addresses.len(), 1);
        assert!(addresses[0].address_info.is_some());
        assert_eq!(
            addresses[0].address_info.as_ref().unwrap()[0].local,
            Some(address.into())
        );
        assert_eq!(
            addresses[0].address_info.as_ref().unwrap()[0].broadcast,
            Some(broadcast_address.into())
        );
    }

    #[tokio::test]
    async fn test_delete() {
        let link_name = "test_addr1";
        let address = "172.80.0.2";
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

        client
            .address()
            .add(AddressAddConfiguration {
                local: address.into(),
                device: link_name.into(),
                ..Default::default()
            })
            .await
            .unwrap();

        client
            .address()
            .delete(AddressDeleteConfiguration {
                local: address.into(),
                device: link_name.into(),
                ..Default::default()
            })
            .await
            .unwrap();

        let addresses = client
            .address()
            .show(Some(AddressShowConfiguration {
                device: link_name.into(),
                ..Default::default()
            }))
            .await
            .unwrap();

        client
            .link()
            .delete(LinkDeleteConfiguration {
                device: LinkDeviceOrGroup::Device(link_name.into()),
                link_type: "dummy".into(),
            })
            .await
            .unwrap();

        assert_eq!(addresses.len(), 1);
        assert!(addresses[0].address_info.as_ref().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_flush() {
        let link_name = "test_addr2";
        let address = "172.80.0.3";
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

        client
            .address()
            .add(AddressAddConfiguration {
                local: address.into(),
                device: link_name.into(),
                ..Default::default()
            })
            .await
            .unwrap();

        client
            .address()
            .flush(Some(AddressFlushConfiguration {
                device: Some(link_name.into()),
                ..Default::default()
            }))
            .await
            .unwrap();

        let addresses = client
            .address()
            .show(Some(AddressShowConfiguration {
                device: link_name.into(),
                ..Default::default()
            }))
            .await
            .unwrap();

        client
            .link()
            .delete(LinkDeleteConfiguration {
                device: LinkDeviceOrGroup::Device(link_name.into()),
                link_type: "dummy".into(),
            })
            .await
            .unwrap();

        assert_eq!(addresses.len(), 1);
        assert!(addresses[0].address_info.as_ref().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_save_and_restore() {
        let link_name = "test_addr3";
        let address = "172.80.0.4";
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

        client
            .address()
            .add(AddressAddConfiguration {
                local: address.into(),
                device: link_name.into(),
                ..Default::default()
            })
            .await
            .unwrap();

        let netlink_configuration = client
            .address()
            .save(Some(AddressSaveConfiguration {
                device: Some(link_name.into()),
                ..Default::default()
            }))
            .await
            .unwrap();

        client
            .address()
            .flush(Some(AddressFlushConfiguration {
                device: Some(link_name.into()),
                ..Default::default()
            }))
            .await
            .unwrap();

        client
            .address()
            .restore(netlink_configuration.clone())
            .await
            .unwrap();

        let addresses = client
            .address()
            .show(Some(AddressShowConfiguration {
                device: link_name.into(),
                ..Default::default()
            }))
            .await
            .unwrap();

        client
            .link()
            .delete(LinkDeleteConfiguration {
                device: LinkDeviceOrGroup::Device(link_name.into()),
                link_type: "dummy".into(),
            })
            .await
            .unwrap();

        assert!(!netlink_configuration.is_empty());
        assert_eq!(addresses.len(), 1);
        assert!(addresses[0].address_info.is_some());
        assert_eq!(
            addresses[0].address_info.as_ref().unwrap()[0].local,
            Some(address.into())
        );
    }
}
