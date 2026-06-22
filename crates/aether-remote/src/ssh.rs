use std::sync::mpsc;
use std::path::Path;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;

use crate::remote_fs::{RemoteFs, RemoteDirEntry, FsEvent, Result};
use ssh2::Session as SshSession;

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

/// SSH 远程文件系统实现
pub struct SshRemoteFs {
    config: SshConfig,
    session: Option<SshSession>,
}

impl SshRemoteFs {
    /// 创建新的 SSH 远程文件系统
    pub fn new(config: SshConfig) -> Self {
        Self {
            config,
            session: None,
        }
    }

    /// 建立 SSH 连接
    pub fn connect(&mut self) -> Result<()> {
        let tcp = TcpStream::connect(format!("{}:{}", self.config.host, self.config.port))
            .map_err(|e| format!("TCP 连接失败: {}", e))?;
        
        let mut sess = SshSession::new()
            .map_err(|e| format!("SSH 会话创建失败: {}", e))?;
        
        sess.set_tcp_stream(tcp);
        sess.handshake()
            .map_err(|e| format!("SSH 握手失败: {}", e))?;
        
        // 根据认证方式配置
        match &self.config.auth {
            SshAuth::Password(password) => {
                sess.userauth_password(&self.config.username, password)
                    .map_err(|e| format!("密码认证失败: {}", e))?;
            }
            SshAuth::Key { path, passphrase } => {
                let key_path = Path::new(path);
                if key_path.exists() {
                    match passphrase {
                        Some(pass) => {
                            sess.userauth_pubkey_file(&self.config.username, None, key_path, Some(pass))
                                .map_err(|e| format!("密钥认证失败: {}", e))?;
                        }
                        None => {
                            sess.userauth_pubkey_file(&self.config.username, None, key_path, None)
                                .map_err(|e| format!("密钥认证失败: {}", e))?;
                        }
                    }
                } else {
                    return Err(format!("密钥文件不存在: {}", path));
                }
            }
            SshAuth::Agent => {
                sess.userauth_agent(&self.config.username)
                    .map_err(|e| format!("Agent 认证失败: {}", e))?;
            }
        }
        
        self.session = Some(sess);
        Ok(())
    }

    /// 检查连接是否活跃
    pub fn is_connected(&self) -> bool {
        self.session.is_some()
    }

    /// 断开 SSH 连接
    pub fn disconnect(&mut self) {
        self.session = None;
    }

    /// 执行远程命令并返回输出
    fn exec_command(&self, command: &str) -> Result<(String, String)> {
        let sess = self.session.as_ref()
            .ok_or("SSH 未连接，请先调用 connect()")?;
        
        let mut channel = sess.channel_session()
            .map_err(|e| format!("创建通道失败: {}", e))?;
        
        channel.exec(command)
            .map_err(|e| format!("执行命令失败: {}", e))?;
        
        let mut stdout = String::new();
        let mut stderr = String::new();
        channel.read_to_string(&mut stdout)
            .map_err(|e| format!("读取输出失败: {}", e))?;
        
        channel.stderr().read_to_string(&mut stderr)
            .map_err(|e| format!("读取错误输出失败: {}", e))?;
        
        channel.wait_close()
            .map_err(|e| format!("等待命令完成失败: {}", e))?;
        
        Ok((stdout, stderr))
    }
}

impl RemoteFs for SshRemoteFs {
    /// 读取远程文件内容
    fn read_file(&self, path: &str) -> Result<Vec<u8>> {
        let sess = self.session.as_ref()
            .ok_or("SSH 未连接，请先调用 connect()")?;
        
        let (mut channel, _stat) = sess.scp_recv(Path::new(path))
            .map_err(|e| format!("打开 SCP 通道失败: {}", e))?;
        
        let mut content = Vec::new();
        channel.read_to_end(&mut content)
            .map_err(|e| format!("读取文件失败: {}", e))?;
        
        Ok(content)
    }

    /// 写入文件到远程
    fn write_file(&self, path: &str, content: &[u8]) -> Result<()> {
        let sess = self.session.as_ref()
            .ok_or("SSH 未连接，请先调用 connect()")?;
        
        let mut channel = sess.scp_send(Path::new(path), 0o644, content.len() as u64, None)
            .map_err(|e| format!("打开 SCP 发送通道失败: {}", e))?;
        
        channel.write_all(content)
            .map_err(|e| format!("写入文件失败: {}", e))?;
        
        channel.send_eof()
            .map_err(|e| format!("发送 EOF 失败: {}", e))?;
        
        channel.wait_eof()
            .map_err(|e| format!("等待 EOF 失败: {}", e))?;
        
        channel.wait_close()
            .map_err(|e| format!("等待关闭失败: {}", e))?;
        
        Ok(())
    }

    /// 列出远程目录内容
    fn list_dir(&self, path: &str) -> Result<Vec<RemoteDirEntry>> {
        let (stdout, _) = self.exec_command(&format!("ls -la {}", path))?;
        
        let entries: Vec<RemoteDirEntry> = stdout
            .lines()
            .skip(1) // 跳过第一行 "total ..."
            .filter_map(|line| parse_ls_line(line))
            .collect();

        Ok(entries)
    }

    /// 监听文件变更（SSH 场景下可能不适用）
    fn watch(&self, _path: &str) -> Result<mpsc::Receiver<FsEvent>> {
        let (_tx, rx) = mpsc::channel();
        Ok(rx)
    }

    /// 在远程执行命令
    fn exec(&self, command: &str) -> Result<(String, String)> {
        self.exec_command(command)
    }
}

/// 解析 ls -l 输出的一行
fn parse_ls_line(line: &str) -> Option<RemoteDirEntry> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 8 {
        return None;
    }

    let is_dir = parts[0].starts_with('d');
    let size = parts[4].parse::<u64>().unwrap_or(0);
    let name = parts[8..].join(" "); // 处理文件名包含空格的情况

    Some(RemoteDirEntry {
        name,
        is_dir,
        size,
        modified: None,
    })
}