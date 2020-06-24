//!
//! # Fluvio SC - output processing
//!
//! Format SPU Group response based on output type

use prettytable::Row;
use prettytable::row;
use prettytable::Cell;
use prettytable::cell;
use prettytable::format::Alignment;
use log::debug;

use flv_client::metadata::objects::Metadata;
use flv_client::metadata::spg::SpuGroupSpec;


use crate::error::CliError;
use crate::output::OutputType;
use crate::TableOutputHandler;
use crate::Terminal;
use crate::t_println;


type ListSpuGroups = Vec<Metadata<SpuGroupSpec>>;

// -----------------------------------
// Format Output
// -----------------------------------

/// Format SPU Group based on output type
pub fn spu_group_response_to_output<O: Terminal>(
    out: std::sync::Arc<O>,
    list_spu_groups: ListSpuGroups,
    output_type: OutputType,
) -> Result<(), CliError> {
   
    debug!("groups: {:#?}", list_spu_groups);

    if list_spu_groups.len() > 0 {
        out.render_list(&list_spu_groups, output_type)
    } else {
        t_println!(out, "no groups");
        Ok(())
    }
}

// -----------------------------------
// Output Handlers
// -----------------------------------
impl TableOutputHandler for ListSpuGroups {
    /// table header implementation
    fn header(&self) -> Row {
        row!["NAME", "REPLICAS", "MIN ID", "RACK", "SIZE", "STATUS",]
    }

    /// return errors in string format
    fn errors(&self) -> Vec<String> {
        self.iter().map(|_g| "".to_owned()).collect()
    }

    /// table content implementation
    fn content(&self) -> Vec<Row> {
        self.iter()
            .map(|r| {
                Row::new(vec![
                    Cell::new_align(&r.name(), Alignment::RIGHT),
                    Cell::new_align(&r.replicas(), Alignment::CENTER),
                    Cell::new_align(&r.min_id(), Alignment::RIGHT),
                    Cell::new_align(&r.rack(), Alignment::RIGHT),
                    Cell::new_align(&r.size(), Alignment::RIGHT),
                    Cell::new_align(&r.status(), Alignment::RIGHT),
                ])
            })
            .collect()
    }
}
