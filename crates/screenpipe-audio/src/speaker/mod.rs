pub mod embedding;

use std::path::Path;

use anyhow::Result;

pub fn create_session<P: AsRef<Path>>(path: P) -> Result<ort::session::Session> {
    let session_result = ort::session::Session::builder()?
        .with_optimization_level(ort::session::builder::GraphOptimizationLevel::Level3)?
        .with_intra_threads(1)?
        .with_inter_threads(1)?
        .commit_from_file(path.as_ref());
        
    match session_result {
        Ok(session) => Ok(session),
        Err(e) => {
            // Sentry issue 7244944243: If the ONNX file is corrupted (e.g. Protobuf parsing failed),
            // delete it so it can be automatically re-downloaded by the next model refresh.
            if let Err(rm_err) = std::fs::remove_file(path.as_ref()) {
                tracing::warn!("failed to remove potentially corrupted model file {:?}: {}", path.as_ref(), rm_err);
            } else {
                tracing::info!("removed potentially corrupted model file {:?}", path.as_ref());
            }
            Err(e.into())
        }
    }
}
pub mod embedding_manager;
pub mod models;
mod prepare_segments;
pub use prepare_segments::prepare_segments;
pub mod segment;
