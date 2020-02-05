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

use crate::{Error, IpCommand};

#[derive(Clone)]
pub struct IpRouteCommand<'l> {
    ip_command: &'l IpCommand,
}

impl<'l> IpRouteCommand<'l> {
    pub fn new(ip_command: &'l IpCommand) -> Self {
        Self { ip_command }
    }

    /// List routes.
    pub async fn list(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Flush routing tables.
    pub async fn flush(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Save routing table information as raw netlink configuration.
    pub async fn save(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Restore routing table information from raw netlink configuration.
    pub async fn restore(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Convert the raw netlink configuration into a human readable form or json.
    pub async fn show_dump(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Get a single route.
    pub async fn get(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Add new route.
    pub async fn add(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Delete route.
    pub async fn delete(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Change route.
    pub async fn change(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Change or add new route.
    pub async fn replace(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Append a new route.
    pub async fn append(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Prepend a new route.
    pub async fn prepend(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
