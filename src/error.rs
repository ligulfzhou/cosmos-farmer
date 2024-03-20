use bip39::Error as Bip39Error;
use cosmrs::bip32::Error as Bip32Error;
use cosmrs::rpc::Error as TendermintRPCError;
use cosmrs::tendermint::Error as TendermintError;
use cosmrs::Error as CosmrsError;
use eyre::Report as EyreError;
use reqwest::Error as ReqwestError;
use thiserror::Error;
use tonic::{transport::Error as GRPCError, Status as StatusError};

pub type MyResult<T> = Result<T, MyError>;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("bip39: {:?}", .0)]
    Bip39Error(#[from] Bip39Error),

    #[error("CosmrsError: {:?}", .0)]
    CosmrsError(#[from] CosmrsError),

    #[error("RPC: {:?}", .0)]
    RPCError(String),

    #[error("Bip32Error: {:?}", .0)]
    Bip32Error(#[from] Bip32Error),

    #[error("TendermintRPCError: {:?}", .0)]
    TendermintRPCError(#[from] TendermintRPCError),

    #[error("TendermintError: {:?}", .0)]
    TendermintError(#[from] TendermintError),

    #[error("ReqwestError: {:?}", .0)]
    ReqwestError(#[from] ReqwestError),

    #[error("GRPCError: {:?}", .0)]
    GRPCError(#[from] GRPCError),

    #[error("StatusError: {:?}", .0)]
    StatusError(#[from] StatusError),

    #[error("GRPCMessageFailed: {:?}", .0)]
    GRPCMessageFailed(String),

    #[error("EyreError: {:?}", .0)]
    EyreError(#[from] EyreError),
}
