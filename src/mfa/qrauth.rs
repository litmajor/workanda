use qrcode::QrCode;
use std::io::Cursor;

pub fn generate_qr_code(username: &str, secret: &str) -> Result<Vec<u8>, String> {
    let otpauth_url = format!(
        "otpauth://totp/Workanda:{}?secret={}&issuer=Workanda",
        username, secret
    );

    let qr_code = QrCode::new(otpauth_url).map_err(|e| e.to_string())?;
    let mut buffer = Cursor::new(Vec::new());
    qr_code.write_png(&mut buffer).map_err(|e| e.to_string())?;
    Ok(buffer.into_inner())
}