use std::time::SystemTime;
use std::sync::mpsc;

/// 远程目录条目
#[derive(Clone, Debug)]
pub struct RemoteDirEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

/// 文件系统事件
#[derive(Clone, Debug)]
pub enum FsEvent {
    Created { path: String },
    Modified { path: String },
    Deleted { path: String },
    Renamed { from: String, to: String },
}

/// 远程文件系统结果类型
pub type Result<T> = std::result::Result<T, String>;

/// 远程文件系统抽象 trait
/// 统一SSH、容器等远程环境的文件访问接口
pub trait RemoteFs: Send + Sync {
    /// 读取文件内容
    fn read_file(&self, path: &str) -> Result<Vec<u8>>;

    /// 写入文件内容
    fn write_file(&self, path: &str, content: &[u8]) -> Result<()>;

    /// 列出目录内容
    fn list_dir(&self, path: &str) -> Result<Vec<RemoteDirEntry>>;

    /// 监听文件变更（如果后端支持）
    fn watch(&self, path: &str) -> Result<mpsc::Receiver<FsEvent>>;

    /// 在远程执行命令
    fn exec(&self, command: &str) -> Result<(String, String)>;

    /// 检查路径是否存在
    fn exists(&self, path: &str) -> Result<bool> {
        match self.read_file(path) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
