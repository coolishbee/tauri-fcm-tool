use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;

/// 메시지 템플릿
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub id: String,
    pub name: String,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Template {
    /// 새 템플릿 생성
    pub fn new(name: String, title: String, body: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            title,
            body,
            created_at: now,
            updated_at: now,
        }
    }

    /// 템플릿 업데이트
    pub fn update(&mut self, name: String, title: String, body: String) {
        self.name = name;
        self.title = title;
        self.body = body;
        self.updated_at = Utc::now();
    }
}

/// 템플릿 목록
#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TemplateList {
    pub templates: Vec<Template>,
}

impl TemplateList {
    /// 템플릿 추가 또는 업데이트
    pub fn save(&mut self, template: Template) {
        if let Some(existing) = self.templates.iter_mut().find(|t| t.id == template.id) {
            *existing = template;
        } else {
            self.templates.push(template);
        }
    }

    /// 템플릿 삭제
    pub fn delete(&mut self, id: &str) -> bool {
        let len_before = self.templates.len();
        self.templates.retain(|t| t.id != id);
        self.templates.len() != len_before
    }

    /// ID로 템플릿 찾기
    pub fn get(&self, id: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.id == id)
    }
}
