pub enum Event {
    PackageLoadedSuccess(Vec<String>),
    PackageLoadedFailed(String),
    PackageUpdate,
    PackageUpdateSuccess,
    PackageUpdateFailed(String),
}
