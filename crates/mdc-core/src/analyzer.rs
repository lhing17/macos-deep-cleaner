use crate::scanner::ScanItem;
use mdc_rules::{Category, SafetyLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub total_items: usize,
    pub total_size: u64,
    pub by_category: HashMap<Category, CategorySummary>,
    pub by_safety: HashMap<SafetyLevel, u64>,
    pub top_items: Vec<ScanItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySummary {
    pub count: usize,
    pub size: u64,
}

pub struct Analyzer;

impl Analyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, items: &[ScanItem], top_n: usize) -> AnalysisReport {
        let total_items = items.len();
        let total_size = items.iter().filter_map(|i| i.size).sum();

        let mut by_category: HashMap<Category, CategorySummary> = HashMap::new();
        let mut by_safety: HashMap<SafetyLevel, u64> = HashMap::new();

        for item in items {
            let size = item.size.unwrap_or(0);
            by_category
                .entry(item.category)
                .and_modify(|s| {
                    s.count += 1;
                    s.size += size;
                })
                .or_insert(CategorySummary { count: 1, size });

            by_safety
                .entry(item.safety)
                .and_modify(|s| *s += size)
                .or_insert(size);
        }

        let mut sorted = items.to_vec();
        sorted.sort_by(|a, b| b.size.unwrap_or(0).cmp(&a.size.unwrap_or(0)));
        let top_items = sorted.into_iter().take(top_n).collect();

        AnalysisReport {
            total_items,
            total_size,
            by_category,
            by_safety,
            top_items,
        }
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    if size == 0 {
        return "0 B".to_string();
    }
    let exp = (size as f64).log(1024.0).min(UNITS.len() as f64 - 1.0) as usize;
    let value = size as f64 / 1024f64.powi(exp as i32);
    format!("{:.2} {}", value, UNITS[exp])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_item(path: &str, category: Category, safety: SafetyLevel, size: u64) -> ScanItem {
        ScanItem {
            path: PathBuf::from(path),
            rule_name: "test".to_string(),
            category,
            safety,
            size: Some(size),
        }
    }

    #[test]
    fn test_analyze_totals() {
        let items = vec![
            make_item("/a", Category::SystemCache, SafetyLevel::Safe, 100),
            make_item("/b", Category::SystemLogs, SafetyLevel::Safe, 200),
            make_item("/c", Category::DevNode, SafetyLevel::Caution, 300),
        ];
        let analyzer = Analyzer::new();
        let report = analyzer.analyze(&items, 10);

        assert_eq!(report.total_items, 3);
        assert_eq!(report.total_size, 600);
        assert_eq!(report.by_category.len(), 3);
        assert_eq!(report.top_items.len(), 3);
        assert_eq!(report.top_items[0].path, PathBuf::from("/c"));
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512.00 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }
}
