use crate::{Category, Rule, SafetyLevel};
use std::path::PathBuf;

/// 构建内置的 macOS 清理规则集
pub fn macos_builtin_rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));

    // ==================== 通用系统垃圾 ====================
    rules.push(
        Rule::new("User Caches", Category::SystemCache, SafetyLevel::Safe)
            .with_dirs(vec![home.join("Library").join("Caches")])
            .with_description("User-level application caches")
    );

    rules.push(
        Rule::new("System Caches", Category::SystemCache, SafetyLevel::Caution)
            .with_dirs(vec![PathBuf::from("/Library/Caches")])
            .with_description("System-level application caches (may require sudo)")
    );

    rules.push(
        Rule::new("User Logs", Category::SystemLogs, SafetyLevel::Safe)
            .with_dirs(vec![home.join("Library").join("Logs")])
            .with_description("User application logs")
    );

    rules.push(
        Rule::new("System Logs", Category::SystemLogs, SafetyLevel::Caution)
            .with_dirs(vec![PathBuf::from("/var/log")])
            .with_description("System logs (may require sudo)")
    );

    rules.push(
        Rule::new("DS_Store Files", Category::DsStore, SafetyLevel::Safe)
            .with_dirs(vec![
                home.join("Desktop"),
                home.join("Documents"),
                home.join("Downloads"),
                home.join("Projects"),
                home.join("Developer"),
            ])
            .with_patterns(vec![".DS_Store".to_string()])
            .with_max_depth(10)
            .with_description("macOS .DS_Store files in common directories")
    );

    rules.push(
        Rule::new("Trash", Category::Trash, SafetyLevel::Caution)
            .with_dirs(vec![home.join(".Trash")])
            .with_description("Files in the Trash bin")
    );

    // ==================== 应用残留 ====================
    rules.push(
        Rule::new("App Support Leftovers", Category::AppLeftovers, SafetyLevel::Caution)
            .with_dirs(vec![home.join("Library/Application Support")])
            .with_description("Application support directories (heuristic detection planned)")
    );

    rules.push(
        Rule::new("Preferences Leftovers", Category::AppLeftovers, SafetyLevel::Caution)
            .with_dirs(vec![home.join("Library/Preferences")])
            .with_description("Preference plist files from uninstalled apps")
    );

    rules.push(
        Rule::new("Containers Leftovers", Category::AppLeftovers, SafetyLevel::Caution)
            .with_dirs(vec![home.join("Library/Containers")])
            .with_description("Sandboxed app container leftovers")
    );

    // ==================== 开发环境残留 ====================
    rules.push(
        Rule::new("Homebrew Cache", Category::DevHomebrew, SafetyLevel::Safe)
            .with_dirs(vec![
                home.join("Library/Caches/Homebrew"),
                PathBuf::from("/opt/homebrew/Library/Caches/Homebrew"),
                PathBuf::from("/usr/local/Homebrew/Library/Caches/Homebrew"),
            ])
            .with_description("Homebrew download caches")
    );

    rules.push(
        Rule::new("npm Cache", Category::DevNode, SafetyLevel::Safe)
            .with_dirs(vec![home.join(".npm")])
            .with_description("npm global cache")
    );

    rules.push(
        Rule::new("yarn Cache", Category::DevNode, SafetyLevel::Safe)
            .with_dirs(vec![home.join(".yarn/cache"), home.join(".cache/yarn")])
            .with_description("Yarn global cache")
    );

    rules.push(
        Rule::new("pnpm Store", Category::DevNode, SafetyLevel::Safe)
            .with_dirs(vec![home.join(".local/share/pnpm/store"), home.join(".pnpm-store")])
            .with_description("pnpm global store")
    );

    rules.push(
        Rule::new("Python pip Cache", Category::DevPython, SafetyLevel::Safe)
            .with_dirs(vec![home.join(".cache/pip")])
            .with_description("pip download cache")
    );

    rules.push(
        Rule::new("Python PyCache", Category::DevPython, SafetyLevel::Safe)
            .with_dirs(vec![
                home.join("Projects"),
                home.join("Developer"),
                home.join("Documents"),
            ])
            .with_patterns(vec!["__pycache__".to_string()])
            .with_max_depth(10)
            .with_description("Python __pycache__ directories in common project folders")
    );

    rules.push(
        Rule::new("Xcode DerivedData", Category::DevXcode, SafetyLevel::Safe)
            .with_dirs(vec![home.join("Library/Developer/Xcode/DerivedData")])
            .with_description("Xcode build intermediates")
    );

    rules.push(
        Rule::new("Xcode Archives", Category::DevXcode, SafetyLevel::Caution)
            .with_dirs(vec![home.join("Library/Developer/Xcode/Archives")])
            .with_description("Xcode app archives")
    );

    rules.push(
        Rule::new("iOS Simulator Data", Category::DevXcode, SafetyLevel::Caution)
            .with_dirs(vec![home.join("Library/Developer/CoreSimulator/Devices")])
            .with_description("iOS Simulator device data")
    );

    rules.push(
        Rule::new("JetBrains Caches", Category::DevJetBrains, SafetyLevel::Safe)
            .with_dirs(vec![home.join("Library/Caches/JetBrains")])
            .with_description("JetBrains IDE caches")
    );

    rules.push(
        Rule::new("JetBrains Logs", Category::DevJetBrains, SafetyLevel::Safe)
            .with_dirs(vec![home.join("Library/Logs/JetBrains")])
            .with_description("JetBrains IDE logs")
    );

    rules.push(
        Rule::new("Gradle Cache", Category::DevGeneral, SafetyLevel::Safe)
            .with_dirs(vec![home.join(".gradle/caches")])
            .with_description("Gradle build caches")
    );

    rules.push(
        Rule::new("Cargo Registry Cache", Category::DevGeneral, SafetyLevel::Safe)
            .with_dirs(vec![home.join(".cargo/registry/cache")])
            .with_description("Cargo crate download cache")
    );

    rules.push(
        Rule::new("Rustup Temp", Category::DevGeneral, SafetyLevel::Safe)
            .with_dirs(vec![home.join(".rustup/tmp")])
            .with_description("Rustup temporary files")
    );

    rules
}
