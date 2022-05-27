
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[cfg(feature = "ApplicationModel")]
pub mod ApplicationModel;
#[cfg(feature = "Devices")]
pub mod Devices;
#[cfg(feature = "Management")]
pub mod Management;
#[cfg(feature = "Media")]
pub mod Media;
#[cfg(feature = "Notification")]
pub mod Notification;
#[cfg(feature = "PersonalInformation")]
pub mod PersonalInformation;
#[cfg(feature = "Speech")]
pub mod Speech;
#[cfg(feature = "StartScreen")]
pub mod StartScreen;
#[cfg(feature = "System")]
pub mod System;
#[cfg(feature = "UI")]
pub mod UI;
