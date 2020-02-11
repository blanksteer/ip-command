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

use crate::{Error, IpCommand};

#[derive(Clone)]
pub struct IpRuleCommand<'l> {
    ip_command: &'l IpCommand,
}

impl<'l> IpRuleCommand<'l> {
    pub fn new(ip_command: &'l IpCommand) -> Self {
        Self { ip_command }
    }

    /// Insert a new rule.
    pub async fn add(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Delete a rule.
    pub async fn delete(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Flush rules table information.
    pub async fn flush(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Save rules table information as raw netlink configuration.
    pub async fn save(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Restore rules table information from raw netlink configuration.
    pub async fn restore(&self) -> Result<(), Error> {
        unimplemented!()
    }

    // List rules.
    pub async fn list(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
