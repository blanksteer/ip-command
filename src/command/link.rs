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
pub struct IpLinkCommand<'l> {
    ip_command: &'l IpCommand,
}

impl<'l> IpLinkCommand<'l> {
    pub fn new(ip_command: &'l IpCommand) -> Self {
        Self { ip_command }
    }

    /// Add virtual link.
    pub async fn add(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Delete virtual link.
    pub async fn delete(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Change device attributes.
    pub async fn set(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Display device attributes.
    pub async fn show(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Display extended statistics.
    pub async fn xstats(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Display address-family specific statistics.
    pub async fn afstats(&self) -> Result<(), Error> {
        unimplemented!()
    }
}