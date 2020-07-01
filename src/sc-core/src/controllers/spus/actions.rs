//!
//! # SPU Actions
//!
//!  SPU action definition and processing handlers
//!
use std::fmt;

use flv_metadata::spu::SpuSpec;
use flv_metadata::k8::metadata::ObjectMeta;
use flv_metadata::store::actions::*;
use flv_util::actions::Actions;

use crate::controllers::conn_manager::SpuConnectionStatusChange;
use crate::controllers::conn_manager::ConnectionRequest;

use super::SpuLSChange;

#[derive(Debug, PartialEq, Clone)]
pub enum SpuChangeRequest {
    SpuLS(Actions<SpuLSChange>),
    Conn(SpuConnectionStatusChange),
}

impl fmt::Display for SpuChangeRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpuChangeRequest::SpuLS(req) => write!(f, "SPU LS: {}", req.count()),
            SpuChangeRequest::Conn(req) => write!(f, "Conn LS: {}", req),
        }
    }
}

#[derive(Debug, Default)]
pub struct SpuActions {
    pub spus: Actions<WSAction<SpuSpec,ObjectMeta>>,
    pub conns: Actions<ConnectionRequest>,
}

impl fmt::Display for SpuActions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SPU WS: {}, Conn Mgr:: {}, ",
            self.spus.count(),
            self.conns.count(),
        )
    }
}
