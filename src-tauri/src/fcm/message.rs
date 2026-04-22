use serde::{Deserialize, Serialize};
use specta::Type;

/// FCM 메시지 발송 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// 단일 디바이스
    Single,
    /// 토픽
    Topic,
}

/// FCM 메시지 내용
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FcmMessage {
    pub title: String,
    pub body: String,
}

/// Android 알림 설정
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AndroidSettings {
    /// 우선순위: "high" 또는 "normal"
    pub priority: String,
    /// 알림 채널 ID
    pub channel_id: String,
}

/// APNs (iOS) 알림 설정
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ApnsSettings {
    /// 우선순위: "10" (high) 또는 "5" (normal)
    pub priority: String,
    /// 사운드: "default" 또는 커스텀 사운드 파일명
    #[serde(default)]
    pub sound: Option<String>,
}

/// FCM 발송 요청
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SendRequest {
    pub message_type: MessageType,
    pub message: FcmMessage,
    /// 단일 디바이스용 토큰
    #[serde(default)]
    pub token: Option<String>,
    /// 토픽명
    #[serde(default)]
    pub topic: Option<String>,
    /// Android 설정
    #[serde(default)]
    pub android: Option<AndroidSettings>,
    /// APNs (iOS) 설정
    #[serde(default)]
    pub apns: Option<ApnsSettings>,
}

/// FCM 발송 결과
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SendResult {
    pub success: bool,
    pub details: String,
}

impl SendResult {
    pub fn success(details: String) -> Self {
        Self {
            success: true,
            details,
        }
    }

    pub fn failure(details: String) -> Self {
        Self {
            success: false,
            details,
        }
    }
}
