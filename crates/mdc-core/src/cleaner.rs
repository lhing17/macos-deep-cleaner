use crate::scanner::ScanItem;
use anyhow::Result;
use std::path::Path;

/// 清理模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanMode {
    /// 仅预览，不执行任何删除
    DryRun,
    /// 直接删除（移到废纸篓）
    MoveToTrash,
}

pub struct Cleaner;

impl Cleaner {
    pub fn new() -> Self {
        Self
    }

    /// 清理给定的项目列表
    pub fn clean(&self, items: &[ScanItem], mode: CleanMode) -> Result<Vec<CleanResult>> {
        let mut results = Vec::with_capacity(items.len());
        for item in items {
            let result = match mode {
                CleanMode::DryRun => CleanResult {
                    path: item.path.clone(),
                    success: true,
                    message: "[DRY-RUN] would be moved to trash".to_string(),
                },
                CleanMode::MoveToTrash => self.move_to_trash(&item.path),
            };
            results.push(result);
        }
        Ok(results)
    }

    fn move_to_trash(&self, path: &Path) -> CleanResult {
        match trash::delete(path) {
            Ok(_) => CleanResult {
                path: path.to_path_buf(),
                success: true,
                message: "Moved to trash".to_string(),
            },
            Err(e) => CleanResult {
                path: path.to_path_buf(),
                success: false,
                message: format!("Failed: {}", e),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct CleanResult {
    pub path: std::path::PathBuf,
    pub success: bool,
    pub message: String,
}

impl Default for Cleaner {
    fn default() -> Self {
        Self::new()
    }
}
