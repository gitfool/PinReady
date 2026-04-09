use std::io::BufReader;
use std::path::{Path, PathBuf};

const CACHE_FILENAME: &str = ".pinready_bg.png";

/// Get the cached backglass image path for a table directory.
pub fn cached_bg_path(table_dir: &Path) -> PathBuf {
    table_dir.join(CACHE_FILENAME)
}

/// Extract the backglass image from a .directb2s file and cache it as PNG.
/// Returns the path to the cached image, or None if extraction failed.
pub fn extract_backglass(directb2s_path: &Path, table_dir: &Path) -> Option<PathBuf> {
    let cache_path = cached_bg_path(table_dir);

    // Already cached?
    if cache_path.exists() {
        return Some(cache_path);
    }

    log::info!("Extracting backglass from {}", directb2s_path.display());

    let file = match std::fs::File::open(directb2s_path) {
        Ok(f) => f,
        Err(e) => { log::error!("Failed to open {}: {e}", directb2s_path.display()); return None; }
    };
    let reader = BufReader::new(file);
    let data = match directb2s::read(reader) {
        Ok(d) => d,
        Err(e) => { log::error!("Failed to parse {}: {e}", directb2s_path.display()); return None; }
    };

    // Try BackglassImage first (full quality), fall back to ThumbnailImage
    let b64_data = if let Some(ref bg) = data.images.backglass_image {
        &bg.value
    } else {
        &data.images.thumbnail_image.value
    };

    if b64_data.is_empty() || b64_data == "[stripped]" {
        log::warn!("No backglass image data in {}", directb2s_path.display());
        return None;
    }

    log::info!("Decoding base64 ({} chars)...", b64_data.len());

    // Strip whitespace/newlines from base64 data (directb2s files contain \r\n in base64)
    use base64::Engine;
    let clean_b64: String = b64_data.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = match base64::engine::general_purpose::STANDARD.decode(&clean_b64) {
        Ok(b) => b,
        Err(e) => { log::error!("Base64 decode failed: {e}"); return None; }
    };

    log::info!("Decoded {} bytes, loading image...", bytes.len());

    // Load as image
    let img = match image::load_from_memory(&bytes) {
        Ok(i) => i,
        Err(e) => { log::error!("Image decode failed for {}: {e}", directb2s_path.display()); return None; }
    };

    // Crop out the grill/DMD area at the bottom using GrillHeight
    let grill_height: u32 = data.grill_height.value.parse().unwrap_or(0);
    let crop_height = if grill_height > 0 && grill_height < img.height() {
        img.height() - grill_height
    } else {
        img.height()
    };
    let cropped = img.crop_imm(0, 0, img.width(), crop_height);

    // Resize to reasonable width for the launcher (keep aspect ratio)
    let resized = cropped.resize(400, 300, image::imageops::FilterType::Lanczos3);
    if let Err(e) = resized.save(&cache_path) {
        log::error!("Failed to save cache {}: {e}", cache_path.display());
        return None;
    }

    log::info!("Cached backglass: {} ({}x{})", cache_path.display(), resized.width(), resized.height());
    Some(cache_path)
}
