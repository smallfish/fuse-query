// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use log::info;
use simplelog::{Config, LevelFilter, SimpleLogger};

use std::sync::{Arc, Mutex};
use tokio::signal::unix::{signal, SignalKind};

use fuse_query::contexts::Options;
use fuse_query::datasources::DataSource;
use fuse_query::servers::MySQLHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Options::default();

    match opts.log_level.to_lowercase().as_str() {
        "debug" => SimpleLogger::init(LevelFilter::Debug, Config::default())?,
        "info" => SimpleLogger::init(LevelFilter::Info, Config::default())?,
        _ => SimpleLogger::init(LevelFilter::Error, Config::default())?,
    }
    info!("{:?}", opts.clone());

    let datasource = DataSource::try_create()?;
    let mysql_handler = MySQLHandler::create(opts.clone(), Arc::new(Mutex::new(datasource)));
    tokio::spawn(async move { mysql_handler.start() });

    info!("Fuse-Query Cloud Compute Starts...");
    info!("Usage: mysql -h127.0.0.1 -P{:?}", opts.mysql_handler_port);
    signal(SignalKind::hangup())?.recv().await;
    Ok(())
}
