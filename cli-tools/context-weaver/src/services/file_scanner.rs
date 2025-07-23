use anyhow::Result;
use dashmap::DashMap;
use ignore::gitignore::GitignoreBuilder;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;

use crate::models::FileInfo;

#[derive(Clone)]
pub struct FileScanner {
    root_path: PathBuf,
    file_map: Arc<DashMap<String, FileInfo>>,
}

impl FileScanner {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            file_map: Arc::new(DashMap::new()),
        }
    }

    pub fn scan(&self) -> Result<()> {
        let wvignore_path = self.root_path.join(".wvignore");
        let mut gitignore = GitignoreBuilder::new(&self.root_path);

        if wvignore_path.exists() {
            gitignore.add(&wvignore_path);
        }

        let gitignore = gitignore.build()?;

        for entry in WalkDir::new(&self.root_path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                let path = e.path();
                if path == self.root_path {
                    return true;
                }

                let relative_path = path.strip_prefix(&self.root_path).unwrap_or(path);
                !gitignore.matched(relative_path, path.is_dir()).is_ignore()
            })
        {
            let entry = entry?;
            let path = entry.path();

            if let Ok(relative_path) = path.strip_prefix(&self.root_path) {
                let path_str = relative_path.to_string_lossy().to_string();

                let preview = if path.is_file() {
                    self.get_file_preview(path)?
                } else {
                    String::new()
                };

                let file_info = FileInfo {
                    path: path_str.clone(),
                    preview,
                    is_directory: path.is_dir(),
                };

                self.file_map.insert(path_str, file_info);
            }
        }

        Ok(())
    }

    fn get_file_preview(&self, path: &Path) -> Result<String> {
        if let Ok(content) = fs::read_to_string(path) {
            let first_line = content.lines().next().unwrap_or("");
            Ok(first_line.chars().take(30).collect())
        } else {
            Ok(String::from("[Binary file]"))
        }
    }

    pub fn get_file_map(&self) -> &DashMap<String, FileInfo> {
        &self.file_map
    }

    pub fn resolve_includes(
        &self,
        narrative_data: &crate::models::NarrativeData,
    ) -> Result<String> {
        let mut result = String::new();

        for context_item in &narrative_data.contexts {
            let full_path = self.root_path.join(&context_item.path);

            if !full_path.exists() {
                anyhow::bail!("File not found: {}", context_item.path);
            }

            let content = fs::read_to_string(&full_path)?;

            match &context_item.include_type {
                crate::models::IncludeType::Full => {
                    result.push_str(&content);
                }
                crate::models::IncludeType::Section { section } => {
                    // 簡易的な実装: #section_name から次の # までを抽出
                    let section_marker = format!("# {section}");
                    if let Some(start_idx) = content.find(&section_marker) {
                        let section_content = &content[start_idx..];
                        if let Some(end_idx) = section_content[1..].find("\n#") {
                            result.push_str(&section_content[..end_idx + 1]);
                        } else {
                            result.push_str(section_content);
                        }
                    }
                }
                crate::models::IncludeType::Lines { start, end } => {
                    let lines: Vec<&str> = content.lines().collect();
                    let start_idx = start.saturating_sub(1);
                    let end_idx = (*end).min(lines.len());

                    for line in &lines[start_idx..end_idx] {
                        result.push_str(line);
                        result.push('\n');
                    }
                }
            }

            result.push_str("\n\n");
        }

        Ok(result)
    }
}
