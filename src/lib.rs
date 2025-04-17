use binaryninja::binary_view::{BinaryView, BinaryViewExt};
use binaryninja::rc::Ref;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EError {
    #[error("Binary Ninja init error")]
    BinjaInitError(#[from] binaryninja::headless::InitializationError),

    #[error("Could not load file with headless Binary Ninja")]
    BinjaProjectLoadError,
}

pub type EResult<T> = Result<T, EError>;

pub fn binja_load_into_session(
    session: &binaryninja::headless::Session,
    path: String,
) -> EResult<Ref<BinaryView>> {
    let binary_view = session.load(&path).ok_or(EError::BinjaProjectLoadError)?;
    binary_view.update_analysis_and_wait();
    Ok(binary_view)
}

pub fn binja_load(path: String) -> EResult<Ref<BinaryView>> {
    let headless_session = binaryninja::headless::Session::new()?;
    let binary_view = headless_session
        .load(&path)
        .ok_or(EError::BinjaProjectLoadError)?;
    binary_view.update_analysis_and_wait();
    Ok(binary_view)
}
