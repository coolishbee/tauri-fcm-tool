use serde::Deserialize;
use std::time::Duration;

use crate::modules::logger;

const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

/// Google OAuth 토큰 교환 응답
#[derive(Debug, Deserialize)]
pub struct ExchangeCodeResponse {
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: String,
    #[serde(default)]
    pub id_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

/// Google OAuth refresh token 응답
#[derive(Debug, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

/// Google OAuth 토큰 엔드포인트를 통해 authorization code를 token으로 교환
pub async fn exchange_code_with_google(
    client_id: &str,
    client_secret: Option<&str>,
    code: &str,
    redirect_uri: &str,
    code_verifier: &str,
) -> Result<ExchangeCodeResponse, String> {
    logger::info("Google OAuth 토큰 교환 요청 시작");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("HTTP 클라이언트 생성 실패: {}", e))?;

    let mut form = vec![
        ("client_id", client_id.to_string()),
        ("code", code.to_string()),
        ("redirect_uri", redirect_uri.to_string()),
        ("grant_type", "authorization_code".to_string()),
        ("code_verifier", code_verifier.to_string()),
    ];

    if let Some(secret) = client_secret.filter(|secret| !secret.is_empty()) {
        form.push(("client_secret", secret.to_string()));
    }

    let resp = client
        .post(GOOGLE_TOKEN_URL)
        .form(&form)
        .send()
        .await
        .map_err(|e| format!("토큰 교환 요청 실패: {}", e))?;

    let status = resp.status();
    let body = resp
        .text()
        .await
        .map_err(|e| format!("응답 읽기 실패: {}", e))?;

    logger::info(&format!(
        "Google OAuth 토큰 교환 응답 수신: HTTP {}",
        status.as_u16()
    ));

    if !status.is_success() {
        logger::error_with_context(
            "oauth_exchange",
            &format!("토큰 교환 실패: HTTP {} | {}", status.as_u16(), body),
        );
        return Err(format!(
            "토큰 교환 실패: HTTP {} | {}",
            status.as_u16(),
            body
        ));
    }

    let token_response: ExchangeCodeResponse = serde_json::from_str(&body)
        .map_err(|e| format!("응답 파싱 실패: {} | body: {}", e, body))?;

    if token_response.access_token.is_empty() {
        logger::error_with_context("oauth_exchange", "응답에 access_token이 없습니다");
        return Err("응답에 access_token이 없습니다".to_string());
    }

    logger::info("Google OAuth 토큰 교환 성공");

    Ok(token_response)
}

/// Google OAuth 토큰 엔드포인트를 통해 refresh token으로 access token을 갱신
pub async fn refresh_access_token_with_google(
    client_id: &str,
    client_secret: Option<&str>,
    refresh_token: &str,
) -> Result<RefreshTokenResponse, String> {
    logger::info("Google OAuth refresh token 갱신 요청 시작");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("HTTP 클라이언트 생성 실패: {}", e))?;

    let mut form = vec![
        ("client_id", client_id.to_string()),
        ("refresh_token", refresh_token.to_string()),
        ("grant_type", "refresh_token".to_string()),
    ];

    if let Some(secret) = client_secret.filter(|secret| !secret.is_empty()) {
        form.push(("client_secret", secret.to_string()));
    }

    let resp = client
        .post(GOOGLE_TOKEN_URL)
        .form(&form)
        .send()
        .await
        .map_err(|e| format!("토큰 갱신 요청 실패: {}", e))?;

    let status = resp.status();
    let body = resp
        .text()
        .await
        .map_err(|e| format!("응답 읽기 실패: {}", e))?;

    logger::info(&format!(
        "Google OAuth refresh token 응답 수신: HTTP {}",
        status.as_u16()
    ));

    if !status.is_success() {
        logger::error_with_context(
            "oauth_refresh",
            &format!("토큰 갱신 실패: HTTP {} | {}", status.as_u16(), body),
        );
        return Err(format!(
            "토큰 갱신 실패: HTTP {} | {}",
            status.as_u16(),
            body
        ));
    }

    let token_response: RefreshTokenResponse = serde_json::from_str(&body)
        .map_err(|e| format!("응답 파싱 실패: {} | body: {}", e, body))?;

    if token_response.access_token.is_empty() {
        logger::error_with_context("oauth_refresh", "응답에 access_token이 없습니다");
        return Err("응답에 access_token이 없습니다".to_string());
    }

    logger::info("Google OAuth refresh token 갱신 성공");

    Ok(token_response)
}
