pub mod builtins;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 清理项的安全等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SafetyLevel {
    /// 低风险，通常是缓存/日志
    Safe,
    /// 中风险，配置文件或应用数据
    Caution,
    /// 高风险，可能仍在使用
    Danger,
}

impl std::fmt::Display for SafetyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SafetyLevel::Safe => write!(f, "Safe"),
            SafetyLevel::Caution => write!(f, "Caution"),
            SafetyLevel::Danger => write!(f, "Danger"),
        }
    }
}

/// 垃圾类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    SystemCache,
    SystemLogs,
    AppLeftovers,
    DevHomebrew,
    DevNode,
    DevDocker,
    DevPython,
    DevXcode,
    DevJetBrains,
    DevGeneral,
    Trash,
    DsStore,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Category::SystemCache => "System Cache",
            Category::SystemLogs => "System Logs",
            Category::AppLeftovers => "App Leftovers",
            Category::DevHomebrew => "Dev - Homebrew",
            Category::DevNode => "Dev - Node.js",
            Category::DevDocker => "Dev - Docker",
            Category::DevPython => "Dev - Python",
            Category::DevXcode => "Dev - Xcode",
            Category::DevJetBrains => "Dev - JetBrains",
            Category::DevGeneral => "Dev - General",
            Category::Trash => "Trash",
            Category::DsStore => ".DS_Store",
        };
        write!(f, "{}", s)
    }
}

/// 一条清理规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub category: Category,
    pub safety: SafetyLevel,
    /// 需要扫描的基目录
    pub base_dirs: Vec<PathBuf>,
    /// 可选：只匹配特定文件名/目录名（支持通配符简化逻辑，后续可升级 glob）
    pub patterns: Vec<String>,
    /// 可选：递归扫描的最大深度，None 表示无限
    pub max_depth: Option<usize>,
    /// 规则描述
    pub description: String,
}

impl Rule {
    pub fn new(name: impl Into<String>, category: Category, safety: SafetyLevel) -> Self {
        Self {
            name: name.into(),
            category,
            safety,
            base_dirs: Vec::new(),
            patterns: Vec::new(),
            max_depth: None,
            description: String::new(),
        }
    }

    pub fn with_dirs(mut self, dirs: Vec<PathBuf>) -> Self {
        self.base_dirs = dirs;
        self
    }

    pub fn with_patterns(mut self, patterns: Vec<String>) -> Self {
        self.patterns = patterns;
        self
    }

    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}

/// 规则集合
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleSet {
    pub rules: Vec<Rule>,
}

impl RuleSet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    pub fn by_category(&self, category: Category) -> Vec<&Rule> {
        self.rules
            .iter()
            .filter(|r| r.category == category)
            .collect()
    }
}
