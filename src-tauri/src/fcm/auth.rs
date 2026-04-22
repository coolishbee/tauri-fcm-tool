use chrono::{DateTime, Duration, Utc};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

use super::exchange::{
    exchange_code_with_google, refresh_access_token_with_google, ExchangeCodeResponse,
    RefreshTokenResponse,
};
use super::pkce::{generate_code_challenge, generate_code_verifier, generate_state};
use crate::modules::logger;

const FCM_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";
const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";

/// OAuth 토큰 정보
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OAuthToken {
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: String,
    #[serde(default)]
    pub id_token: String,
    pub token_type: String,
    pub expiry: DateTime<Utc>,
}

impl OAuthToken {
    /// 토큰이 유효한지 확인
    pub fn is_valid(&self) -> bool {
        !self.access_token.is_empty() && Utc::now() < self.expiry
    }

    /// ExchangeCodeResponse로부터 OAuthToken 생성
    pub fn from_response(resp: ExchangeCodeResponse) -> Self {
        let expiry = Utc::now() + Duration::seconds(resp.expires_in);
        Self {
            access_token: resp.access_token,
            refresh_token: resp.refresh_token,
            id_token: resp.id_token,
            token_type: resp.token_type,
            expiry,
        }
    }

    /// RefreshTokenResponse로부터 기존 refresh token을 유지한 OAuthToken 생성
    pub fn from_refresh_response(resp: RefreshTokenResponse, refresh_token: String) -> Self {
        let expiry = Utc::now() + Duration::seconds(resp.expires_in);
        Self {
            access_token: resp.access_token,
            refresh_token,
            id_token: String::new(),
            token_type: resp.token_type,
            expiry,
        }
    }
}

/// OAuth 인증 결과
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AuthResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub token: Option<OAuthToken>,
}

impl AuthResult {
    pub fn success(token: OAuthToken) -> Self {
        Self {
            success: true,
            message: "인증 성공".to_string(),
            token: Some(token),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            token: None,
        }
    }
}

/// OAuth 인증 URL 생성
pub fn build_auth_url(
    client_id: &str,
    redirect_url: &str,
    state: &str,
    code_challenge: &str,
) -> String {
    format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}&code_challenge={}&code_challenge_method=S256&access_type=offline&prompt=consent",
        GOOGLE_AUTH_URL,
        urlencoding::encode(client_id),
        urlencoding::encode(redirect_url),
        urlencoding::encode(FCM_SCOPE),
        urlencoding::encode(state),
        urlencoding::encode(code_challenge),
    )
}

fn callback_bind_addr_and_path(redirect_url: &str) -> Result<(String, String), String> {
    let parsed =
        Url::parse(redirect_url).map_err(|e| format!("리다이렉트 URL 파싱 실패: {}", e))?;
    let host = parsed
        .host_str()
        .ok_or_else(|| "리다이렉트 URL에 호스트가 없습니다".to_string())?;
    let port = parsed
        .port_or_known_default()
        .ok_or_else(|| "리다이렉트 URL에 포트가 없습니다".to_string())?;
    let path = if parsed.path().is_empty() {
        "/".to_string()
    } else {
        parsed.path().to_string()
    };

    Ok((format!("{}:{}", host, port), path))
}

/// OAuth 콜백 서버 시작 및 인증 코드 수신
pub fn start_oauth_callback_server(
    expected_state: &str,
    redirect_url: &str,
) -> Result<String, String> {
    logger::info("OAuth 루프백 콜백 서버 시작");

    let (bind_addr, expected_path) = callback_bind_addr_and_path(redirect_url)?;
    logger::info(&format!(
        "OAuth 루프백 서버 바인딩: {}{}",
        bind_addr, expected_path
    ));

    let listener =
        TcpListener::bind(&bind_addr).map_err(|e| format!("콜백 서버 바인딩 실패: {}", e))?;

    listener
        .set_nonblocking(false)
        .map_err(|e| format!("서버 설정 실패: {}", e))?;

    let (tx, rx) = mpsc::channel();
    let state = expected_state.to_string();

    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut request_line = String::new();

            if reader.read_line(&mut request_line).is_ok() {
                // GET /callback?code=xxx&state=yyy HTTP/1.1
                if let Some(target) = request_line.split_whitespace().nth(1) {
                    let (path, query) = target.split_once('?').unwrap_or((target, ""));

                    if path != expected_path {
                        logger::error_with_context(
                            "oauth_callback",
                            &format!("예상하지 않은 콜백 경로 수신: {}", path),
                        );

                        let response_body = r#"<html><head><meta charset="utf-8"></head><body style="font-family: Arial; text-align: center; padding: 50px;"><h1 style="color: #f44336;">인증 실패</h1><p>잘못된 콜백 경로입니다.</p></body></html>"#;
                        let response = format!(
                            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            response_body.len(),
                            response_body
                        );
                        let _ = stream.write_all(response.as_bytes());
                        let _ = tx.send(Err("잘못된 콜백 경로".to_string()));
                        return;
                    }

                    let params: Vec<&str> =
                        query.split('&').filter(|param| !param.is_empty()).collect();

                    let mut code = None;
                    let mut received_state = None;
                    let mut error = None;

                    for param in params {
                        let kv: Vec<&str> = param.splitn(2, '=').collect();
                        if kv.len() == 2 {
                            let decoded_value = urlencoding::decode(kv[1])
                                .map(|v| v.into_owned())
                                .unwrap_or_else(|_| kv[1].to_string());

                            match kv[0] {
                                "code" => code = Some(decoded_value),
                                "state" => received_state = Some(decoded_value),
                                "error" => error = Some(decoded_value),
                                _ => {}
                            }
                        }
                    }

                    let (response_body, result) = if let Some(err) = error {
                        logger::error_with_context(
                            "oauth_callback",
                            &format!("OAuth 오류 파라미터 수신: {}", err),
                        );
                        (
                            format!(
                                r#"<html><head><meta charset="utf-8"></head><body style="font-family: Arial; text-align: center; padding: 50px;"><h1 style="color: #f44336;">인증 실패</h1><p>오류: {}</p></body></html>"#,
                                err
                            ),
                            Err(format!("OAuth 오류: {}", err)),
                        )
                    } else if received_state.as_ref() != Some(&state) {
                        logger::error_with_context("oauth_callback", "State 검증 실패");
                        (
                            r#"<html><head><meta charset="utf-8"></head><body style="font-family: Arial; text-align: center; padding: 50px;"><h1 style="color: #f44336;">인증 실패</h1><p>State 검증 실패 (보안 오류)</p></body></html>"#.to_string(),
                            Err("State 불일치".to_string()),
                        )
                    } else if let Some(auth_code) = code {
                        logger::info("OAuth 콜백에서 authorization code 수신");
                        (
                            r#"<html><head><meta charset="utf-8"></head><body style="font-family: Arial; text-align: center; padding: 50px;"><h1 style="color: #4CAF50;">인증 코드 수신 완료</h1><p>앱에서 Google 토큰 교환을 진행 중입니다. 완료될 때까지 잠시 기다린 뒤 이 창을 닫으세요.</p></body></html>"#.to_string(),
                            Ok(auth_code),
                        )
                    } else {
                        logger::error_with_context("oauth_callback", "인증 코드가 없습니다");
                        (
                            r#"<html><head><meta charset="utf-8"></head><body style="font-family: Arial; text-align: center; padding: 50px;"><h1 style="color: #f44336;">인증 실패</h1><p>인증 코드가 없습니다.</p></body></html>"#.to_string(),
                            Err("인증 코드 없음".to_string()),
                        )
                    };

                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        response_body.len(),
                        response_body
                    );

                    let _ = stream.write_all(response.as_bytes());
                    let _ = tx.send(result);
                }
            }
        }
    });

    // 5분 타임아웃
    rx.recv_timeout(std::time::Duration::from_secs(300))
        .map_err(|_| "인증 타임아웃".to_string())?
}

pub async fn authenticate(client_id: &str, client_secret: &str, redirect_url: &str) -> AuthResult {
    logger::info("OAuth 인증 플로우 시작");

    // 1. PKCE 파라미터 생성
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);
    let state = generate_state();

    // 2. 인증 URL 생성
    let auth_url = build_auth_url(client_id, redirect_url, &state, &code_challenge);

    // 3. 브라우저 열기
    if let Err(e) = open::that(&auth_url) {
        logger::error_with_context("oauth_authenticate", &format!("브라우저 열기 실패: {}", e));
        return AuthResult::failure(format!("브라우저 열기 실패: {}", e));
    }

    logger::info("OAuth 인증 URL을 브라우저로 열기 성공");

    // 4. 콜백 서버 시작 및 인증 코드 수신
    let code = match start_oauth_callback_server(&state, redirect_url) {
        Ok(code) => code,
        Err(e) => {
            logger::error_with_context("oauth_authenticate", &format!("콜백 처리 실패: {}", e));
            return AuthResult::failure(e);
        }
    };

    logger::info("OAuth authorization code 수신 후 토큰 교환 시작");

    // 5. 토큰 교환
    match exchange_code_with_google(
        client_id,
        Some(client_secret),
        &code,
        redirect_url,
        &code_verifier,
    )
    .await
    {
        Ok(response) => {
            logger::info("OAuth 토큰 교환 완료");
            let token = OAuthToken::from_response(response);
            AuthResult::success(token)
        }
        Err(e) => {
            logger::error_with_context("oauth_authenticate", &format!("토큰 교환 실패: {}", e));
            AuthResult::failure(format!("토큰 교환 실패: {}", e))
        }
    }
}

/// 저장된 refresh token으로 access token 갱신
pub async fn refresh_oauth_token(
    client_id: &str,
    client_secret: &str,
    token: &OAuthToken,
) -> Result<OAuthToken, String> {
    if token.refresh_token.is_empty() {
        logger::warn("refresh token 없이 토큰 갱신 시도됨");
        return Err("refresh token이 없습니다".to_string());
    }

    logger::info("저장된 refresh token으로 access token 갱신 시도");
    let response =
        refresh_access_token_with_google(client_id, Some(client_secret), &token.refresh_token)
            .await?;
    Ok(OAuthToken::from_refresh_response(
        response,
        token.refresh_token.clone(),
    ))
}
