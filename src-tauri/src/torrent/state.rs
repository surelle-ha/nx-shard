use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use anyhow::Context;
use tokio::sync::RwLock;
use librqbit::{ManagedTorrent, Session};

use crate::configs::defaults::get_game_path; 

pub struct TorrentState {
    pub session: Arc<Session>,
    pub handles: Arc<RwLock<HashMap<u32, (usize, Arc<ManagedTorrent>)>>>,
}

impl TorrentState {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let session = Session::new(Path::new(&get_game_path()).to_path_buf())
            .await
            .context("error creating shared session")?;

        Ok(Self {
            session, 
            handles: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}