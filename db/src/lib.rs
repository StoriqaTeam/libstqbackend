//! This crate provides common utilities for DB interaction.
#[macro_use]
extern crate failure;
extern crate futures;
extern crate futures_state_stream;
extern crate tokio_postgres;

pub mod connection;
pub mod statement;
pub mod repo;
