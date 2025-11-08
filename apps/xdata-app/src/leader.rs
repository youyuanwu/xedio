use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use kube_leader_election::{LeaseLock, LeaseLockParams};
use rand::{Rng, distr::Alphanumeric};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct LeaderElection {
    pub is_leader: Arc<AtomicBool>,
}

impl Default for LeaderElection {
    fn default() -> Self {
        Self::new()
    }
}

impl LeaderElection {
    pub fn new() -> Self {
        LeaderElection {
            is_leader: Arc::new(AtomicBool::new(false)),
        }
    }
    pub async fn election_loop(self, token: CancellationToken) {
        let client = kube::Client::try_default().await.unwrap();

        // random id part for the sake of simulating something like a pod hash
        let random: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let holder_id = format!("shared-lease-{}", random.to_lowercase());

        let leadership = LeaseLock::new(
            client,
            "xedio",
            LeaseLockParams {
                holder_id,
                lease_name: "shared-lease-example".into(),
                lease_ttl: Duration::from_secs(15),
            },
        );

        loop {
            tokio::select! {
                result = leadership.try_acquire_or_renew() => {
                    match result {
                        Ok(ll) => self.is_leader.store(ll.acquired_lease, Ordering::Relaxed),
                        Err(err) => tracing::error!("Leader election error {:?}", err),
                    }
                }
                _ = token.cancelled() => {
                    tracing::info!("Leader election loop received shutdown signal");
                    break;
                }
            }
            // sleep before next renew attempt
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(5)) => {},
                _ = token.cancelled() => {
                    tracing::info!("Leader election loop received shutdown signal");
                    break;
                }
            }
        }
        // step down as leader
        match leadership.step_down().await {
            Ok(_) => {
                tracing::info!("Stepped down from leadership");
                self.is_leader.store(false, Ordering::Relaxed);
            }
            Err(err) => tracing::error!("Error stepping down from leadership: {:?}", err),
        }
    }
}

// gRPC service implementation
use tonic::{Request, Response, Status};

use crate::proto::hello_world;
pub use hello_world::leader_election_server::{
    LeaderElection as LeaderElectionService, LeaderElectionServer,
};
use hello_world::{IsLeaderReply, IsLeaderRequest};

#[derive(Debug, Clone)]
pub struct MyLeaderElection {
    leader_state: LeaderElection,
}

impl MyLeaderElection {
    pub fn new(leader_state: LeaderElection) -> Self {
        Self { leader_state }
    }
}

#[tonic::async_trait]
impl LeaderElectionService for MyLeaderElection {
    async fn is_leader(
        &self,
        _request: Request<IsLeaderRequest>,
    ) -> Result<Response<IsLeaderReply>, Status> {
        // Get pod name from environment variable (set by Kubernetes)
        let pod_name = std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());

        let is_leader = self.leader_state.is_leader.load(Ordering::Relaxed);

        tracing::debug!(
            "Leader status check: pod={}, is_leader={}",
            pod_name,
            is_leader
        );

        let reply = IsLeaderReply {
            is_leader,
            leader_id: pod_name,
        };

        Ok(Response::new(reply))
    }
}
