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

use super::rule::IpRuleCommand;
use crate::{Error, IpCommand};

#[derive(Clone)]
pub struct IpMulticastRuleCommand<'l> {
    ip_rule_command: IpRuleCommand<'l>,
}

// TODO investigate multicast argument passing
impl<'l> IpMulticastRuleCommand<'l> {
    pub fn new(ip_command: &'l IpCommand) -> Self {
        let ip_rule_command = IpRuleCommand::new(ip_command);
        Self { ip_rule_command }
    }

    /// Insert a new multicast rule.
    pub async fn add(&self) -> Result<(), Error> {
        self.ip_rule_command.add().await
    }

    /// Delete a multicast rule.
    pub async fn delete(&self) -> Result<(), Error> {
        self.ip_rule_command.delete().await
    }

    /// Flush multicast rules table information.
    pub async fn flush(&self) -> Result<(), Error> {
        self.ip_rule_command.flush().await
    }

    /// Save multicast rules table information as raw netlink configuration.
    pub async fn save(&self) -> Result<(), Error> {
        self.ip_rule_command.save().await
    }

    /// Restore multicast rules table information from raw netlink configuration.
    pub async fn restore(&self) -> Result<(), Error> {
        self.ip_rule_command.restore().await
    }

    // List multicast rules.
    pub async fn list(&self) -> Result<(), Error> {
        self.ip_rule_command.list().await
    }
}
