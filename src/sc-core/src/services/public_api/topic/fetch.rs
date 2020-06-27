use log::{trace, debug};
use std::io::Error;


use sc_api::objects::*;

use crate::core::SharedContext;
use crate::stores::topic::*;


pub async fn handle_fetch_topics_request(
    filters: Vec<NameFilter>,
    ctx: SharedContext,
) -> Result<ListResponse,Error> {
    

    // encode topics
    let mut topics = ctx.topics().
    // prepare response
    let mut response = FetchTopicsResponse::default();
    response.topics = topics;

    debug!("flv fetch topics resp: {} items", response.topics.len());
    trace!("flv fetch topics resp {:#?}", response);

    Ok(request.new_response(response))
}

/*

/// Encode Topic metadata into a Topic FLV Reponse
pub fn topic_store_metadata_to_topic_response(
    topics: &TopicLocalStore,
    topic_name: &String,
) -> FetchTopicResponse {
    if let Some(topic) = topics.topic(topic_name) {
        FetchTopicResponse::new(
            topic_name.clone(),
            topic.spec.clone(),
            topic.status.clone(),
            None,
        )
    } else {
        FetchTopicResponse::new_not_found(topic_name.clone())
    }
}

/// Encode partitions into a Replica Reponse
pub fn partition_metadata_to_replica_response(
    partitions: &PartitionLocalStore,
    topic: &String,
) -> Vec<PartitionReplica> {
    let mut res: Vec<PartitionReplica> = Vec::default();
    let partition_cnt = partitions.count_topic_partitions(topic);
    for idx in 0..partition_cnt {
        let name = ReplicaKey::new(topic.clone(), idx);
        if let Some(partition) = partitions.value(&name) {
            res.push(PartitionReplica {
                id: idx,
                leader: partition.spec.leader,
                replicas: partition.spec.replicas.clone(),
                live_replicas: partition.status.live_replicas().clone(),
            })
        }
    }
    res
}
*/
