//! Exposed channels which can be used to share data across devices & processors

#[cfg(feature = "split")]
pub use embassy_sync::{blocking_mutex, channel, pubsub, zerocopy_channel};
use embassy_sync::{channel::Channel, rwlock::RwLock};
#[cfg(feature = "_ble")]
use {crate::ble::profile::BleProfileAction, embassy_sync::signal::Signal, rmk_types::led_indicator::LedIndicator};

use crate::hid::Report;
#[cfg(feature = "storage")]
use crate::{FLASH_CHANNEL_SIZE, storage::FlashOperationMessage};
use crate::{REPORT_CHANNEL_SIZE, RawMutex};

/// Signal for LED indicator, used in BLE keyboards only since BLE receiving is not async
#[cfg(feature = "_ble")]
pub(crate) static LED_SIGNAL: Signal<RawMutex, LedIndicator> = Signal::new();
/// Channel for keyboard report from input processors to hid writer/reader
pub static KEYBOARD_REPORT_RECEIVER: Channel<RawMutex, Report, REPORT_CHANNEL_SIZE> = Channel::new();
/// TODO: Make zero-cost without hid_report_proxy feature
pub static KEYBOARD_REPORT_SENDER: RwLock<RawMutex, &'static Channel<RawMutex, Report, REPORT_CHANNEL_SIZE>> =
    RwLock::new(&KEYBOARD_REPORT_RECEIVER);

// Sync messages from server to flash
#[cfg(feature = "storage")]
pub(crate) static FLASH_CHANNEL: Channel<RawMutex, FlashOperationMessage, FLASH_CHANNEL_SIZE> = Channel::new();
#[cfg(feature = "_ble")]
pub(crate) static BLE_PROFILE_CHANNEL: Channel<RawMutex, BleProfileAction, 1> = Channel::new();
