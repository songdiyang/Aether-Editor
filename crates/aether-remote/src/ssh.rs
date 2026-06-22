use std::sync::mpsc;

use crate::remote_fs::{RemoteFs, RemoteDirEntry, FsEvent, Result};

/// SSH 认证方式
#[derive(Clone, Debug)]
pub enum SshAuth {
    Password(String),
    Key { path: String, passphrase: Option<String> },
    Agent,
}

/// SSH 连接配置
#[derive(Clone, Debug)]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth: SshAuth,
}

impl Default for SshConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 22,
            username: String::new(),
            auth: SshAuth::Agent,
        }
    }
}

/// SSH 远程文件系统实现（占位符）
/// 注：完整实现需要 russh 依赖，当前为架构占位
pub struct SshRemoteFs {
    _config: SshConfig,
}

impl SshRemoteFs {
    pub fn new(config: SshConfig) -> Self {
        Self {
            _config: config,
        }
    }
}

impl RemoteFs for SshRemoteFs {
    fn read_file(&self, _path: &str) -> Result<Vec<u8>> {
        Err("SSH read_file not yet implemented".to_string())
    }

    fn write_file(&self, _path: &str, _content: &[u8]) -> Result<()> {
        Err("SSH write_file not yet implemented".to_string())
    }

    fn list_dir(&self, _path: &str) -> Result<Vec<RemoteDirEntry>> {
        Err("SSH list_dir not yet implemented".to_string())
    }

    fn watch(&self, _path: &str) -> Result<mpsc::Receiver<FsEvent>> {
        let (_tx, rx) = mpsc::channel();
        Ok(rx)
    }

    fn exec(&self, _command: &str) -> Result<(String, String)> {
        Err("SSH exec not yet implemented".to_string())
    }
}
