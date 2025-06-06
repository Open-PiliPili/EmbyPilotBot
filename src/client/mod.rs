//! Telegram Bot API client and utilities.
//!
//! This module provides a complete interface for interacting with the Telegram Bot API,
//! including message formatting helpers and a robust client implementation.
//! 
pub mod markdown;
pub mod telegram_client;

pub use markdown::*;
pub use telegram_client::*;
