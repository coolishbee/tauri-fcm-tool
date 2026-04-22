use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::message::{MessageType, SendRequest, SendResult};
use super::OAuthToken;

/// FCM HTTP v1 API 엔드포인트
fn fcm_endpoint(project_id: &str) -> String {
    format!(
        "https://fcm.googleapis.com/v1/projects/{}/messages:send",
        project_id
    )
}

/// FCM API 요청 페이로드
#[derive(Debug, Serialize)]
struct FcmApiRequest {
    message: FcmApiMessage,
}

#[derive(Debug, Serialize)]
struct FcmApiMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
    notification: FcmNotification,
    #[serde(skip_serializing_if = "Option::is_none")]
    android: Option<AndroidConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apns: Option<ApnsConfig>,
}

#[derive(Debug, Serialize)]
struct FcmNotification {
    title: String,
    body: String,
}

/// Android 설정 (FCM API용)
#[derive(Debug, Serialize)]
struct AndroidConfig {
    priority: String,
    notification: AndroidNotification,
}

#[derive(Debug, Serialize)]
struct AndroidNotification {
    channel_id: String,
}

/// APNs 설정 (FCM API용)
#[derive(Debug, Serialize)]
struct ApnsConfig {
    headers: ApnsHeaders,
    payload: ApnsPayload,
}

#[derive(Debug, Serialize)]
struct ApnsHeaders {
    #[serde(rename = "apns-priority")]
    apns_priority: String,
}

#[derive(Debug, Serialize)]
struct ApnsPayload {
    aps: ApnsAps,
}

#[derive(Debug, Serialize)]
struct ApnsAps {
    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<String>,
}

/// FCM API 응답
#[derive(Debug, Deserialize)]
struct FcmApiResponse {
    name: String,
}

/// FCM API 에러 응답
#[derive(Debug, Deserialize)]
struct FcmApiError {
    error: FcmApiErrorDetail,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FcmApiErrorDetail {
    message: String,
    #[serde(default)]
    status: String,
}

/// FCM 클라이언트
pub struct FcmClient {
    http_client: reqwest::Client,
    project_id: String,
    access_token: String,
}

impl FcmClient {
    /// 새 FCM 클라이언트 생성
    pub fn new(project_id: &str, token: &OAuthToken) -> Result<Self, String> {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("HTTP 클라이언트 생성 실패: {}", e))?;

        Ok(Self {
            http_client,
            project_id: project_id.to_string(),
            access_token: token.access_token.clone(),
        })
    }

    /// 메시지 발송
    pub async fn send(&self, request: SendRequest) -> SendResult {
        match request.message_type {
            MessageType::Single => self.send_to_token(&request).await,
            MessageType::Topic => self.send_to_topic(&request).await,
        }
    }

    /// 단일 디바이스에 발송
    async fn send_to_token(&self, request: &SendRequest) -> SendResult {
        let token = match &request.token {
            Some(t) if !t.is_empty() => t,
            _ => return SendResult::failure("디바이스 토큰이 없습니다".to_string()),
        };

        let payload = FcmApiRequest {
            message: FcmApiMessage {
                token: Some(token.clone()),
                topic: None,
                notification: FcmNotification {
                    title: request.message.title.clone(),
                    body: request.message.body.clone(),
                },
                android: request.android.as_ref().map(|a| AndroidConfig {
                    priority: a.priority.clone(),
                    notification: AndroidNotification {
                        channel_id: a.channel_id.clone(),
                    },
                }),
                apns: request.apns.as_ref().map(|a| ApnsConfig {
                    headers: ApnsHeaders {
                        apns_priority: a.priority.clone(),
                    },
                    payload: ApnsPayload {
                        aps: ApnsAps {
                            sound: a.sound.clone(),
                        },
                    },
                }),
            },
        };

        self.send_single_message(payload).await
    }

    /// 토픽에 발송
    async fn send_to_topic(&self, request: &SendRequest) -> SendResult {
        let topic = match &request.topic {
            Some(t) if !t.is_empty() => t,
            _ => return SendResult::failure("토픽 이름이 없습니다".to_string()),
        };

        let payload = FcmApiRequest {
            message: FcmApiMessage {
                token: None,
                topic: Some(topic.clone()),
                notification: FcmNotification {
                    title: request.message.title.clone(),
                    body: request.message.body.clone(),
                },
                android: request.android.as_ref().map(|a| AndroidConfig {
                    priority: a.priority.clone(),
                    notification: AndroidNotification {
                        channel_id: a.channel_id.clone(),
                    },
                }),
                apns: request.apns.as_ref().map(|a| ApnsConfig {
                    headers: ApnsHeaders {
                        apns_priority: a.priority.clone(),
                    },
                    payload: ApnsPayload {
                        aps: ApnsAps {
                            sound: a.sound.clone(),
                        },
                    },
                }),
            },
        };

        self.send_single_message(payload).await
    }

    /// 단일 메시지 발송 (내부 함수)
    async fn send_single_message(&self, payload: FcmApiRequest) -> SendResult {
        let endpoint = fcm_endpoint(&self.project_id);

        let response = match self
            .http_client
            .post(&endpoint)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => return SendResult::failure(format!("HTTP 요청 실패: {}", e)),
        };

        let status = response.status();
        let body = match response.text().await {
            Ok(b) => b,
            Err(e) => return SendResult::failure(format!("응답 읽기 실패: {}", e)),
        };

        if status.is_success() {
            if let Ok(resp) = serde_json::from_str::<FcmApiResponse>(&body) {
                SendResult::success(format!("발송 성공! Message ID: {}", resp.name))
            } else {
                SendResult::success("발송 성공".to_string())
            }
        } else {
            let error_msg = if let Ok(err) = serde_json::from_str::<FcmApiError>(&body) {
                err.error.message
            } else {
                body
            };
            SendResult::failure(format!("발송 실패 ({}): {}", status.as_u16(), error_msg))
        }
    }
}
