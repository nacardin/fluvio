use std::fmt::Debug;

use kf_protocol::derive::{Decode, Encode};
use kf_protocol::Encoder;
use kf_protocol::Decoder;

/// Create Request
#[derive(Encode, Decode, Default, Debug)]
pub struct CreateRequest<S> 
    where S: Encoder + Decoder + Default + Debug
{
    pub name: String,
    pub dry_run: bool,
    pub spec: S   
}

impl<S> CreateRequest<S> 
    where S: Encoder + Decoder + Default + Debug
{

    pub fn new(name: String,dry_run: bool,spec: S) -> Self {
        Self {
            name,
            dry_run,
            spec
        }
    }

}