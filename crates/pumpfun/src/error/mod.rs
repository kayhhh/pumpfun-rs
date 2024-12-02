//! Error types for the Pump.fun SDK.
//!
//! This module defines the `ClientError` enum, which encompasses various error types that can occur when interacting with the Pump.fun program.
//! It includes specific error cases for bonding curve operations, metadata uploads, Solana client errors, and more.
//!
//! The `ClientError` enum provides a comprehensive set of error types to help developers handle and debug issues that may arise during interactions with the Pump.fun program.
//!
//! # Error Types
//!
//! - `BondingCurveNotFound`: The bonding curve account was not found.
//! - `BondingCurveError`: An error occurred while interacting with the bonding curve.
//! - `BorshError`: An error occurred while serializing or deserializing data using Borsh.
//! - `SolanaClientError`: An error occurred while interacting with the Solana RPC client.
//! - `UploadMetadataError`: An error occurred while uploading metadata to IPFS.
//! - `AnchorClientError`: An error occurred while interacting with the Anchor client.
//! - `InvalidInput`: Invalid input parameters were provided.
//! - `InsufficientFunds`: Insufficient funds for a transaction.
//! - `SimulationError`: Transaction simulation failed.
//! - `RateLimitExceeded`: Rate limit exceeded.

use anchor_client::solana_client;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    /// Bonding curve account was not found
    #[error("bonding curve not found")]
    BondingCurveNotFound,
    /// Error related to bonding curve operations
    #[error("bonding curve error: {0}")]
    BondingCurveError(anyhow::Error),
    /// Error deserializing data using Borsh
    #[error("borsh serialization error: {0}")]
    BorshError(std::io::Error),
    /// Error from Solana RPC client
    #[error("solana client error: {0}")]
    SolanaClientError(Box<solana_client::client_error::ClientError>),
    /// Error uploading metadata
    #[error("metadata upload error: {0}")]
    UploadMetadataError(anyhow::Error),
    /// Error from Anchor client
    #[error("anchor client error: {0}")]
    AnchorClientError(anchor_client::ClientError),
    /// Invalid input parameters
    #[error("invalid input: {0}")]
    InvalidInput(anyhow::Error),
    /// Insufficient funds for transaction
    #[error("insufficient funds for transaction")]
    InsufficientFunds,
    /// Transaction simulation failed
    #[error("transaction simulation failed: {0}")]
    SimulationError(String),
    /// Rate limit exceeded
    #[error("rate limit exceeded")]
    RateLimitExceeded,
}
