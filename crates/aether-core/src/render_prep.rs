use std::sync::{Arc, Mutex};
use std::thread;

use crate::lexer::{LexemeSpan, Language};

/// 渲染行数据 - 预计算的行内容和token
#[derive(Clone, Debug)]
pub struct RenderLine {
    pub text: String,
    pub tokens: Vec<LexemeSpan>,
    pub line_idx: usize,
}

/// 并行渲染预处理器
/// 在后台线程中预计算可见行的文本和token，减少主线程渲染负担
pub struct ParallelRenderPrep {
    /// 线程池大小（通常等于CPU核心数）
    thread_count: usize,
}

impl ParallelRenderPrep {
    pub fn new() -> Self {
        Self {
            thread_count: std::thread::available_parallelism()
                .map(|p| p.get())
                .unwrap_or(4)
                .min(8),
        }
    }

    /// 并行预处理可见行的token
    /// 
    /// 将行范围分块，每块由一个线程处理
    pub fn prepare_tokens_parallel(
        &self,
        lines: &[String],
        language: Language,
    ) -> Vec<Vec<LexemeSpan>> {
        if lines.len() < 100 || self.thread_count < 2 {
            // 行数太少，直接单线程处理
            return self.prepare_tokens_single(lines, language);
        }

        let chunk_size = (lines.len() + self.thread_count - 1) / self.thread_count;
        let results = Arc::new(Mutex::new(vec![Vec::new(); lines.len()]));

        let mut handles = Vec::new();
        for thread_idx in 0..self.thread_count {
            let start = thread_idx * chunk_size;
            let end = ((thread_idx + 1) * chunk_size).min(lines.len());
            if start >= end {
                break;
            }

            let chunk_lines: Vec<String> = lines[start..end].to_vec();
            let results_clone = Arc::clone(&results);
            let lang = language;

            let handle = thread::spawn(move || {
                let lexer = lang.create_lexer();
                let mut local_results = Vec::with_capacity(chunk_lines.len());

                for line in &chunk_lines {
                    let tokens = lexer.lex_full(line);
                    local_results.push(tokens);
                }

                // 写回全局结果
                let mut global = results_clone.lock().unwrap();
                for (i, tokens) in local_results.into_iter().enumerate() {
                    global[start + i] = tokens;
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.join();
        }

        Arc::try_unwrap(results)
            .unwrap_or_else(|_| panic!("Failed to unwrap Arc"))
            .into_inner()
            .unwrap_or_else(|_| panic!("Failed to unwrap Mutex"))
    }

    /// 单线程token预处理（fallback）
    fn prepare_tokens_single(
        &self,
        lines: &[String],
        language: Language,
    ) -> Vec<Vec<LexemeSpan>> {
        let lexer = language.create_lexer();
        lines.iter()
            .map(|line| lexer.lex_full(line))
            .collect()
    }
}

impl Default for ParallelRenderPrep {
    fn default() -> Self {
        Self::new()
    }
}

/// 渲染缓存 - 预计算的可见行数据
/// 
/// 避免每帧重复计算行文本和token
pub struct RenderCache {
    /// 缓存的可见行文本
    pub lines: Vec<String>,
    /// 缓存的token数据
    pub token_lines: Vec<Vec<LexemeSpan>>,
    /// 缓存对应的起始行号
    pub start_line: usize,
    /// 缓存对应的结束行号
    pub end_line: usize,
    /// 缓存版本号（用于失效检测）
    pub version: u64,
}

impl RenderCache {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            token_lines: Vec::new(),
            start_line: 0,
            end_line: 0,
            version: 0,
        }
    }

    /// 检查缓存是否有效
    pub fn is_valid(&self, start_line: usize, end_line: usize, version: u64) -> bool {
        self.start_line == start_line && self.end_line == end_line && self.version == version
    }

    /// 更新缓存
    pub fn update(
        &mut self,
        lines: Vec<String>,
        token_lines: Vec<Vec<LexemeSpan>>,
        start_line: usize,
        end_line: usize,
        version: u64,
    ) {
        self.lines = lines;
        self.token_lines = token_lines;
        self.start_line = start_line;
        self.end_line = end_line;
        self.version = version;
    }

    /// 清空缓存
    pub fn clear(&mut self) {
        self.lines.clear();
        self.token_lines.clear();
        self.start_line = 0;
        self.end_line = 0;
        self.version = 0;
    }
}

impl Default for RenderCache {
    fn default() -> Self {
        Self::new()
    }
}
