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
pub struct IpNeighborCommand<'l> {
    ip_command: &'l IpCommand,
}

impl<'l> IpNeighborCommand<'l> {
    pub fn new(ip_command: &'l IpCommand) -> Self {
        Self { ip_command }
    }

    /// Add a new neighbour entry.
    pub async fn add(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Delete a neighbour entry,
    pub async fn delete(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Change an existing entry.
    pub async fn change(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Add a new entry or change an existing one.
    pub async fn replace(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// List neighbour entries.
    pub async fn show(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Flush neighbour entries.
    pub async fn flush(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
