//==============================================================================
// Horizon Game Server - Core Implementation
//==============================================================================
// A high-performance, multithreaded game server using Socket.IO for real-time
// communication. Features include:
//
// - Scalable thread pool architecture supporting up to 32,000 concurrent players
// - Dynamic player connection management with automatic load balancing
// - Integrated plugin system for extensible functionality
// - Comprehensive logging and monitoring
// - Real-time Socket.IO event handling
// - Graceful error handling and connection management
//
// Structure:
// - Player connections are distributed across multiple thread pools
// - Each pool manages up to 1000 players independently
// - Message passing system for inter-thread communication
// - Asynchronous event handling using Tokio runtime
//
// Authors: Tristan James Poland, Thiago M. R. Goulart, Michael Houston
// License: Apache-2.0
//==============================================================================
// #[global_allocator] 
// static ALLOCATOR: dhat::Alloc = dhat::Alloc;
mod config;
mod players;
mod splash;
static CTRL_C_HANDLER: Once = Once::new();
use anyhow::{anyhow, Context, Result as Anyhow};
use viz::{Body, Request, Response, Result};
use horizon_logger::log_info;
use std::{alloc, result::Result::Ok, sync::Once};
use crate::server::CONFIG;
use server::HorizonServer;
use tokio::time::Instant;
use server::LOGGER;
pub mod server;

/// HTTP handler for redirecting browser access to the master panel
async fn redirect_to_master_panel(_req: Request) -> Result<Response<Body>, viz::Error> {
    let response = Response::builder()
        .status(302)
        .header("Location", "https://youtu.be/dQw4w9WgXcQ")
        .body(Body::empty())
        .context("Failed to redict to master panel:")
        .map_err(|e| viz::Error::Boxed(anyhow!("asdfsd").into()))?;

    log_info!(
        LOGGER,
        "HTTP",
        "Browser access redirected to master dashboard"
    );
    Ok(response)
}


/// Main entry point for the Horizon Server
#[tokio::main]
async fn main() -> Anyhow<()> {
    //let mut _profiler = Some(dhat::Profiler::new_heap());
    let init_time = Instant::now();
    let players_per_pool = CONFIG.players_per_pool;
    let num_thread_pools = CONFIG.num_thread_pools;

    // Initialize logging system
    horizon_logger::init();
    splash::splash();
    log_info!(LOGGER, "STARTUP", "Horizon Server starting...");

    // Create and start server instance with configuration values
    let server = HorizonServer::new(players_per_pool, num_thread_pools)?;
    log_info!(
        LOGGER,
        "STARTUP",
        "Server startup completed in {:?}",
        init_time.elapsed()
    );

    let mut terminating: bool = false;
    CTRL_C_HANDLER.call_once(|| {
        // Register the Ctrl+C handler
        ctrlc::set_handler(move ||  {
            if !terminating {
                terminating = true;

                println!("Exit");
                //drop(_profiler.take());
                std::process::exit(0);
                
            }
        },

    ).expect("Failed to handle Ctrl+C")
    });
    server.start().await;


    Ok(())
}
