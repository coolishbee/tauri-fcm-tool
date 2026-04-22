use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;

/// 발송 히스토리 항목
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    /// "single" or "topic"
    pub message_type: String,
    pub title: String,
    pub body: String,
    pub success: bool,
    pub details: String,
}

impl HistoryEntry {
    /// 새 히스토리 항목 생성
    pub fn new(message_type: &str, title: &str, body: &str, success: bool, details: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            message_type: message_type.to_string(),
            title: title.to_string(),
            body: body.to_string(),
            success,
            details: details.to_string(),
        }
    }
}

/// 히스토리 목록 (최대 100개)
#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct HistoryList {
    pub entries: Vec<HistoryEntry>,
    #[serde(default = "default_max_size")]
    pub max_size: u32,
}

fn default_max_size() -> u32 {
    100
}

impl HistoryList {
    /// 새 히스토리 항목 추가 (맨 앞에)
    pub fn add(&mut self, entry: HistoryEntry) {
        self.entries.insert(0, entry);
        // max_size가 0이면 기본값 사용
        let max = if self.max_size == 0 {
            default_max_size() as usize
        } else {
            self.max_size as usize
        };
        if self.entries.len() > max {
            self.entries.truncate(max);
        }
    }

    /// 히스토리 전체 삭제
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// ID로 히스토리 찾기
    pub fn get(&self, id: &str) -> Option<&HistoryEntry> {
        self.entries.iter().find(|e| e.id == id)
    }
}
