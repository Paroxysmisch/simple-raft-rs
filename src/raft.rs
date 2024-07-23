use std::collections::HashSet;
use tokio::sync::mpsc::{Sender, Receiver};

enum Role {
    Leader,
    Follower,
    Candidate
}

struct LogEntry {
    term: usize,
    payload: u64
}

enum Event {
    ElectionTimeout,
    ReplicationTimeout,
    VoteRequest,
    VoteResponse,
    LogRequest,
    LogResponse,
    Broadcast,
}

struct Raft {
    id: u64,
    current_term: u64,
    voted_for: Option<u64>,
    current_role: Role,
    current_leader: Option<u64>,
    votes_received: HashSet<u64>, // Set containing ids of voters

    log: Vec<LogEntry>,
    commit_length: usize,
    sent_length: Vec<usize>, // Amount this node thinks has been sent to each node id
    acked_length: Vec<usize>, // Acknowledgements received from each node id

    send_event_channels: Vec<Sender<Event>>, // Send events to each node id
    recv_event_channel: Receiver<Event>, // Receive events on this node id
    broadcast_channel: Sender<u64>, // Public-facing channel that the current
    // node uses to broadcast to all other nodes
    delivery_channel: Receiver<u64>, // Public-facing channel that the current
    // node uses to receive delivered messages from all nodes (total-order broadcast)
}

impl Raft {
    fn new(id: u64, send_event_channels: &Vec<Sender<Event>>, recv_event_channel: Receiver<Event>) -> Raft {
        let (tx, rx) = tokio::sync::mpsc::channel(32);

        Raft {
            id,
            current_term: 0,
            voted_for: None,
            current_role: Role::Follower,
            current_leader: None,
            votes_received: HashSet::new(),

            log: Vec::new(),
            commit_length: 0,
            sent_length: Vec::with_capacity(send_event_channels.len()),
            acked_length: Vec::with_capacity(send_event_channels.len()),

            send_event_channels: send_event_channels.iter().cloned().collect(),
            recv_event_channel,
            broadcast_channel: tx,
            delivery_channel: rx
        }
    }
}
