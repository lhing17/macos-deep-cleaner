use mdc_rules::{Category, Rule, SafetyLevel};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 扫描结果项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanItem {
    pub path: PathBuf,
    pub rule_name: String,
    pub category: Category,
    pub safety: SafetyLevel,
    pub size: Option<u64>,
}

/// 扫描器
pub struct Scanner;

impl Scanner {
    pub fn new() -> Self {
        Self
    }

    /// 根据规则集执行扫描
    pub fn scan(&self, rules: &[Rule]) -> Vec<ScanItem> {
        rules
            .iter()
            .flat_map(|rule| self.scan_rule(rule))
            .collect()
    }

    fn scan_rule(&self, rule: &Rule) -> Vec<ScanItem> {
        let mut items = Vec::new();
        for base in &rule.base_dirs {
            if !base.exists() {
                continue;
            }
            let walker = if let Some(depth) = rule.max_depth {
                WalkDir::new(base).max_depth(depth)
            } else {
                WalkDir::new(base)
            };

            let mut walker = walker.into_iter();
            while let Some(Ok(entry)) = walker.next() {
                let path = entry.path();
                if self.matches_rule(path, rule) {
                    let size = self.calculate_size(path);
                    items.push(ScanItem {
                        path: path.to_path_buf(),
                        rule_name: rule.name.clone(),
                        category: rule.category,
                        safety: rule.safety,
                        size,
                    });
                    // 如果匹配了目录，不需要继续遍历其子项（避免重复统计）
                    if path.is_dir() {
                        walker.skip_current_dir();
                    }
                }
            }
        }
        items
    }

    fn matches_rule(&self, path: &Path, rule: &Rule) -> bool {
        if rule.patterns.is_empty() {
            // 没有特定模式时，只匹配基目录本身
            rule.base_dirs.iter().any(|b| path == b.as_path())
        } else {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            rule.patterns.iter().any(|p| name == p.as_str())
        }
    }

    fn calculate_size(&self, path: &Path) -> Option<u64> {
        if path.is_file() {
            std::fs::metadata(path).ok().map(|m| m.len())
        } else if path.is_dir() {
            let mut total = 0u64;
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if let Ok(meta) = entry.metadata()
                    && meta.is_file()
                {
                    total += meta.len();
                }
            }
            Some(total)
        } else {
            None
        }
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scan_pattern_files() {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path();
        fs::write(base.join(".DS_Store"), "x").unwrap();
        fs::create_dir(base.join("sub")).unwrap();
        fs::write(base.join("sub").join(".DS_Store"), "yy").unwrap();
        fs::write(base.join("other.txt"), "zzz").unwrap();

        let rule = Rule::new("DS_Store Test", Category::DsStore, SafetyLevel::Safe)
            .with_dirs(vec![base.to_path_buf()])
            .with_patterns(vec![".DS_Store".to_string()]);

        let scanner = Scanner::new();
        let items = scanner.scan(&[rule]);

        assert_eq!(items.len(), 2);
        let total_size: u64 = items.iter().filter_map(|i| i.size).sum();
        assert_eq!(total_size, 3);
    }

    #[test]
    fn test_scan_directory_without_patterns() {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path().join("cache");
        fs::create_dir(&base).unwrap();
        fs::write(base.join("file1.txt"), "hello").unwrap();
        fs::write(base.join("file2.txt"), "world").unwrap();

        let rule = Rule::new("Cache Test", Category::SystemCache, SafetyLevel::Safe)
            .with_dirs(vec![base.clone()]);

        let scanner = Scanner::new();
        let items = scanner.scan(&[rule]);

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].path, base);
        assert_eq!(items[0].size, Some(10));
    }

    #[test]
    fn test_scan_skips_nonexistent_dirs() {
        let rule = Rule::new("Missing", Category::SystemCache, SafetyLevel::Safe)
            .with_dirs(vec![PathBuf::from("/this/path/does/not/exist/12345")]);

        let scanner = Scanner::new();
        let items = scanner.scan(&[rule]);
        assert!(items.is_empty());
    }
}
