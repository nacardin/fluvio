use log::{trace, debug};
use std::io::Error;


use sc_api::objects::*;
use sc_api::topic::TopicSpec;

use crate::core::Context;
use crate::stores::KeyFilter;

pub async fn handle_fetch_topics_request(
    filters: Vec<String>,
    ctx: &Context,
) -> Result<ListResponse,Error> {
    
    let topics: Vec<Metadata<TopicSpec>> = ctx
            .topics()
            .read()
            .values()
            .filter_map(|value| {
                if filters.filter(value.key()) {
                    Some(value.into())
                } else {
                    None
                }  
            })
            .collect();
    


    debug!("flv fetch topics resp: {} items", topics.len());
    trace!("flv fetch topics resp {:#?}", topics);

    Ok(ListResponse::Topic(topics))
}
