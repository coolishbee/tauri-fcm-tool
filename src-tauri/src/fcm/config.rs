use serde::{Deserialize, Serialize};
use specta::Type;

/// FCM 앱 설정
#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FcmConfig {
    /// Google OAuth 2.0 클라이언트 ID
    pub oauth_client_id: String,
    /// OAuth 클라이언트 시크릿 (선택)
    #[serde(default)]
    pub oauth_client_secret: String,
    /// OAuth 리다이렉트 URL
    pub oauth_redirect_url: String,
    /// Firebase 프로젝트 ID
    pub firebase_project_id: String,
}

impl FcmConfig {
    /// 설정이 유효한지 확인
    pub fn is_valid(&self) -> bool {
        !self.oauth_client_id.is_empty()
            && !self.oauth_redirect_url.is_empty()
            && !self.firebase_project_id.is_empty()
    }
}
