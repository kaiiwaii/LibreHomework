mod db;
mod utils;

use tauri::{Event, Manager};
use std::sync::atomic::{AtomicBool, Ordering};
use db::*;
use utils::*;