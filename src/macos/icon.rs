use std::ops::Deref;

/// Icon type for macOS notifications.
///
/// Wraps an icon name or path that is passed to the macOS backend.
/// Construct it from any `&str` via `IconType::from("icon-name")`.
///
/// Общий тип для обоих macOS-бэкендов (`NSUserNotificationCenter` и
/// `UNUserNotificationCenter`), поэтому живёт отдельно от бэкендов.
#[derive(Clone, Debug, Default)]
pub struct IconType(pub String);

impl From<&str> for IconType {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Deref for IconType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
