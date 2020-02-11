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
pub struct IpAddressLabelCommand<'l> {
    ip_command: &'l IpCommand,
}

impl<'l> IpAddressLabelCommand<'l> {
    pub fn new(ip_command: &'l IpCommand) -> Self {
        Self { ip_command }
    }

    /// Add an address label entry to the kernel.
    pub async fn add(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Delete an address label entry from the kernel.
    pub async fn delete(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// List the current address label entries in the kernel.
    pub async fn list(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Flush all address labels in the kernel.
    pub async fn flush(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
