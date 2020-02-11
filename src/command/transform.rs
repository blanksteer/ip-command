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
pub struct IpTransformCommand<'l> {
    ip_command: &'l IpCommand,
}

impl<'l> IpTransformCommand<'l> {
    pub fn new(ip_command: &'l IpCommand) -> Self {
        Self { ip_command }
    }

    /// Add new state into xfrm.
    pub async fn state_add(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Update existing state in xfrm.
    pub async fn state_update(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Allocate an SPI value.
    pub async fn state_allocate_spi(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Delete existing state in xfrm.
    pub async fn state_delete(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Get existing state in xfrm.
    pub async fn state_get(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Delete all existing state in xfrm.
    pub async fn state_delete_all(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Print out the list of existing state in xfrm.
    pub async fn state_list(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Flush all state in xfrm.
    pub async fn state_flush(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Count all existing state in xfrm.
    pub async fn state_count(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Add a new policy.
    pub async fn policy_add(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Update an existing policy.
    pub async fn policy_update(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Delete an existing policy.
    pub async fn policy_delete(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Get an existing policy.
    pub async fn policy_get(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Delete all existing xfrm policies.
    pub async fn policy_delete_all(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Print out the list of xfrm policies.
    pub async fn policy_list(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Flush policies.
    pub async fn policy_flush(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Count existing policies.
    pub async fn policy_count(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Configure the policy hash table.
    pub async fn policy_set(&self) -> Result<(), Error> {
        unimplemented!()
    }

    /// State monitoring for xfrm objects.
    pub async fn monitor(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
