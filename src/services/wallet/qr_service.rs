use qrcode::QrCode;
use image::Luma;
use base64::Engine;
use crate::services::wallet::wallet_service::WalletError;

pub struct QrService;

impl QrService {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_payment_qr(
        &self,
        address: &str,
        currency: &str,
        amount: Option<f64>,
    ) -> Result<String, WalletError> {
        let payment_data = if let Some(amt) = amount {
            format!("celo:{}?currency={}&amount={}", address, currency, amt)
        } else {
            format!("celo:{}?currency={}", address, currency)
        };

        let code = QrCode::new(payment_data.as_bytes())
            .map_err(|e| WalletError::CryptoError(format!("Failed to generate QR code: {}", e)))?;

        let image = code.render::<Luma<u8>>().build();
        
        let mut png_data = Vec::new();
        image.write_to(&mut std::io::Cursor::new(&mut png_data), image::ImageFormat::Png)
            .map_err(|e| WalletError::CryptoError(format!("Failed to encode QR image: {}", e)))?;

        let base64_image = base64::engine::general_purpose::STANDARD.encode(&png_data);
        Ok(format!("data:image/png;base64,{}", base64_image))
    }

    pub fn generate_simple_qr(&self, data: &str) -> Result<String, WalletError> {
        let code = QrCode::new(data.as_bytes())
            .map_err(|e| WalletError::CryptoError(format!("Failed to generate QR code: {}", e)))?;

        let image = code.render::<Luma<u8>>().build();
        
        let mut png_data = Vec::new();
        image.write_to(&mut std::io::Cursor::new(&mut png_data), image::ImageFormat::Png)
            .map_err(|e| WalletError::CryptoError(format!("Failed to encode QR image: {}", e)))?;

        let base64_image = base64::engine::general_purpose::STANDARD.encode(&png_data);
        Ok(format!("data:image/png;base64,{}", base64_image))
    }
}
