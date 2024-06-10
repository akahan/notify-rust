use winrt_notification::Toast;

pub use crate::{error::*, notification::Notification, timeout::Timeout};

use std::{ops, path::Path, str::FromStr};

#[derive(Clone, Debug, Default)]
pub struct IconType(pub String, pub IconCrop);

impl From<String> for IconType {
    fn from(value: String) -> Self {
        Self(value, IconCrop::default())
    }
}

impl From<&str> for IconType {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<(String, IconCrop)> for IconType {
    fn from(value: (String, IconCrop)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<(&str, IconCrop)> for IconType {
    fn from(value: (&str, IconCrop)) -> Self {
        Self::from((value.0.to_string(), value.1))
    }
}

impl ops::Deref for IconType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum IconCrop {
    #[default]
    Square,
    Circular,
}

impl Into<winrt_notification::IconCrop> for IconCrop {
    fn into(self) -> winrt_notification::IconCrop {
        match self {
            IconCrop::Square => winrt_notification::IconCrop::Square,
            IconCrop::Circular => winrt_notification::IconCrop::Circular,
        }
    }
}

pub(crate) fn show_notification(notification: &Notification) -> Result<()> {
    let sound = match &notification.sound_name {
        Some(chosen_sound_name) => winrt_notification::Sound::from_str(chosen_sound_name).ok(),
        None => None,
    };

    let duration = match notification.timeout {
        Timeout::Default => winrt_notification::Duration::Short,
        Timeout::Never => winrt_notification::Duration::Long,
        Timeout::Milliseconds(t) => {
            if t >= 25000 {
                winrt_notification::Duration::Long
            } else {
                winrt_notification::Duration::Short
            }
        }
    };

    let powershell_app_id = &Toast::POWERSHELL_APP_ID.to_string();
    let app_id = &notification.app_id.as_ref().unwrap_or(powershell_app_id);
    let mut toast = Toast::new(app_id)
        .title(&notification.summary)
        .text1(notification.subtitle.as_ref().map_or("", AsRef::as_ref)) // subtitle
        .text2(&notification.body)
        .icon(&Path::new(&notification.icon.0), notification.icon.1.into(), "")
        .sound(sound)
        .duration(duration);
    if let Some(image_path) = &notification.path_to_image {
        toast = toast.image(Path::new(&image_path), "");
    }

    toast
        .show()
        .map_err(|e| Error::from(ErrorKind::Msg(format!("{:?}", e))))
}
