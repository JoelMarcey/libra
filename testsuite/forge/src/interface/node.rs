// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::Result;
use debug_interface::NodeDebugClient;
use diem_config::config::NodeConfig;
use diem_sdk::types::PeerId;

/// A NodeId is intended to be a unique identifier of a Node in a Swarm. Due to VFNs sharing the
/// same PeerId as their Validator, another identifier is needed in order to distinguish between
/// the two.
pub struct NodeId(usize);

/// Trait used to represent a running Validator or FullNode
pub trait Node {
    /// Return the PeerId of this Node
    fn peer_id(&self) -> PeerId;

    /// Return the NodeId of this Node
    fn node_id(&self) -> NodeId;

    /// Return the URL for the JSON-RPC endpoint of this Node
    fn json_rpc_endpoint(&self) -> &str;

    /// Return a NodeDebugClient for this Node
    fn debug_client(&self) -> &NodeDebugClient;

    /// Query a Metric for from this Node
    fn get_metric(&self);

    /// Return a reference to the Config this Node is using
    fn config(&self) -> &NodeConfig;

    /// Start this Node.
    /// This should be a noop if the Node is already running.
    fn start(&mut self) -> Result<()>;

    /// Stop this Node.
    /// This should be a noop if the Node isn't running.
    fn stop(&mut self) -> Result<()>;

    /// Restarts this Node by calling Node::Stop followed by Node::Start
    fn restart(&mut self) -> Result<()> {
        self.stop()?;
        self.start()
    }

    /// Clears this Node's Storage
    fn clear_storage(&mut self) -> Result<()>;

    /// Performs a Health Check on the Node
    fn health_check(&self) -> Result<()>;

    /// Performs a connectivity check on this Node
    fn connectivity_check(&self) -> Result<()>; // Maybe have this as a function as a part of a validator or fullnode
}

/// Trait used to represent a running Validator
pub trait Validator: Node {}

/// Trait used to represent a running FullNode
pub trait FullNode: Node {}
