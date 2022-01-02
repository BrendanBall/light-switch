#![allow(unused_imports)]
#![allow(clippy::single_component_path_imports)]
mod wifi;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Condvar, Mutex};
use std::{cell::RefCell, env, sync::atomic::*, sync::Arc, thread, time::*};

use anyhow::bail;
use anyhow::Result;
use log::*;

use embedded_svc::utils::anyerror::*;

use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::ping;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::wifi::*;

use esp_idf_hal::prelude::*;

use esp_idf_sys;
use esp_idf_sys::esp;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Get backtraces from anyhow; only works for Xtensa arch currently
    // TODO: No longer working with ESP-IDF 4.3.1+
    //#[cfg(target_arch = "xtensa")]
    //env::set_var("RUST_BACKTRACE", "1");


    // #[allow(unused)]
    // let netif_stack = Arc::new(EspNetifStack::new()?);
    // #[allow(unused)]
    // let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    // #[allow(unused)]
    // let default_nvs = Arc::new(EspDefaultNvs::new()?);

    // #[allow(clippy::redundant_clone)]
    // #[cfg(not(feature = "qemu"))]
    // #[allow(unused_mut)]
    // let mut wifi_state = wifi::wifi(
    //     netif_stack.clone(),
    //     sys_loop_stack.clone(),
    //     default_nvs.clone(),
    // );
    // match wifi_state {
    //     Ok(_) => info!("Wifi connected"),
    //     Err(err) => warn!("wifi failed: {}", err)
    // };

    thread::sleep(Duration::from_secs(5));

    info!("hello world");
    Ok(())
}



