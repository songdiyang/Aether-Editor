use std::collections::HashMap;
use std::path::Path;

/// Git 文件状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GitFileStatus {
    Unmodified,     // 未修改
    Modified,       // 已修改
    Added,          // 已暂存
    Deleted,        // 已删除
    Renamed,        // 重命名
    Copied,         // 已复制
    Untracked,      // 未跟踪
    Ignored,        // 已忽略
    Conflict,       // 冲突
}

/// Git 仓库状态
#[derive(Clone, Debug, Default)]
pub struct GitRepository {
    pub is_repo: bool,
    pub branch: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    pub file_status: HashMap<String, GitFileStatus>,
    pub has_changes: bool,
}

impl GitRepository {
    pub fn new() -> Self {
        Self::default()
    }

    /// 检测指定路径是否为 Git 仓库
    pub fn detect(path: &Path) -> Self {
        let mut repo = Self::new();
        
        // 检查 .git 目录是否存在
        let git_dir = path.join(".git");
        if git_dir.exists() {
            repo.is_repo = true;
            repo.branch = Self::get_branch(path);
            repo.file_status = Self::get_status(path);
            repo.has_changes = repo.file_status.values().any(|s| *s != GitFileStatus::Unmodified && *s != GitFileStatus::Ignored);
        }
        
        repo
    }

    /// 获取当前分支名
    fn get_branch(path: &Path) -> Option<String> {
        let head_path = path.join(".git").join("HEAD");
        if let Ok(content) = std::fs::read_to_string(&head_path) {
            let content = content.trim();
            if content.starts_with("ref: refs/heads/") {
                return Some(content[16..].to_string());
            }
            // 分离 HEAD（detached HEAD）
            return Some(content[..7].to_string());
        }
        None
    }

    /// 获取文件状态（简化版：通过 git status --porcelain 解析）
    fn get_status(path: &Path) -> HashMap<String, GitFileStatus> {
        let mut status_map = HashMap::new();
        
        // 尝试执行 git status --porcelain
        if let Ok(output) = std::process::Command::new("git")
            .args(&["status", "--porcelain", "-u"])
            .current_dir(path)
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.len() >= 3 {
                        let status_code = &line[..2];
                        let file_path = &line[3..];
                        
                        let status = match status_code {
                            " M" | "M " | "MM" => GitFileStatus::Modified,
                            "A " | "AM" | "AD" => GitFileStatus::Added,
                            "D " | " D" | "DD" => GitFileStatus::Deleted,
                            "R " | "RM" | "RD" => GitFileStatus::Renamed,
                            "C " | "CM" | "CD" => GitFileStatus::Copied,
                            "??" => GitFileStatus::Untracked,
                            "!!" => GitFileStatus::Ignored,
                            "UU" | "AA" | "AU" | "UA" | "DU" | "UD" => GitFileStatus::Conflict,
                            _ => GitFileStatus::Unmodified,
                        };
                        
                        status_map.insert(file_path.to_string(), status);
                    }
                }
            }
        }
        
        status_map
    }

    /// 刷新状态
    pub fn refresh(&mut self, path: &Path) {
        *self = Self::detect(path);
    }

    /// 获取文件状态
    pub fn file_status(&self, file: &str) -> GitFileStatus {
        self.file_status.get(file).copied().unwrap_or(GitFileStatus::Unmodified)
    }

    /// 获取状态图标
    pub fn status_icon(status: GitFileStatus) -> &'static str {
        match status {
            GitFileStatus::Modified => "M",
            GitFileStatus::Added => "A",
            GitFileStatus::Deleted => "D",
            GitFileStatus::Untracked => "U",
            GitFileStatus::Conflict => "C",
            GitFileStatus::Renamed => "R",
            GitFileStatus::Copied => "C",
            GitFileStatus::Ignored => "I",
            GitFileStatus::Unmodified => "",
        }
    }

    /// 获取状态颜色（用于UI渲染）
    pub fn status_color(status: GitFileStatus) -> (f32, f32, f32) {
        match status {
            GitFileStatus::Modified => (0.9, 0.7, 0.2),  // 黄色
            GitFileStatus::Added => (0.2, 0.8, 0.3),     // 绿色
            GitFileStatus::Deleted => (0.9, 0.2, 0.2),   // 红色
            GitFileStatus::Untracked => (0.5, 0.5, 0.5), // 灰色
            GitFileStatus::Conflict => (0.9, 0.2, 0.9),  // 紫色
            GitFileStatus::Renamed => (0.2, 0.6, 0.9),   // 蓝色
            GitFileStatus::Copied => (0.2, 0.9, 0.9),    // 青色
            GitFileStatus::Ignored => (0.4, 0.4, 0.4),    // 深灰
            GitFileStatus::Unmodified => (0.0, 0.0, 0.0),  // 黑色（不显示）
        }
    }
}

/// Git 集成管理器
pub struct GitIntegration {
    pub repo: GitRepository,
    pub enabled: bool,
}

impl GitIntegration {
    pub fn new() -> Self {
        Self {
            repo: GitRepository::new(),
            enabled: true,
        }
    }

    /// 检测并初始化 Git 仓库
    pub fn detect(&mut self, path: &Path) {
        self.repo = GitRepository::detect(path);
    }

    /// 刷新状态
    pub fn refresh(&mut self, path: &Path) {
        self.repo.refresh(path);
    }

    /// 是否启用了 Git
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// 是否在 Git 仓库中
    pub fn is_repo(&self) -> bool {
        self.repo.is_repo
    }
}

impl Default for GitIntegration {
    fn default() -> Self {
        Self::new()
    }
}
