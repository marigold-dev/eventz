use {
    crate::{
        app_state::AppState,
        config::{Config, PollingInterval, SmartContracts},
        db::{self, model::*, schema::*},
    },
    diesel::prelude::*,
    std::{collections::hash_map::DefaultHasher, error::Error, hash::Hasher, sync::Arc},
    tezos_rpc::{
        client::TezosRpc,
        models::{
            block::{self, Block},
            constants::Constants,
            operation::{
                operation_result::{
                    operations::{event::InternalEventOperationResult, InternalOperationResult},
                    OperationResultStatus,
                },
                OperationContent,
            },
        },
    },
};

const DEFAULT_BLOCK_TIME: i64 = 30;

pub async fn run(app_state: Arc<AppState>, config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    let conn = &mut db::connect::establish_connection(config.clone());
    let rpc = TezosRpc::new(config.tezos_rpc_url.clone());
    let constants = rpc
        .get_constants()
        .block_id(&block::BlockId::Head)
        .send()
        .await?;

    let pooling_interval = if let PollingInterval::Fixed(fixed) = config.pooling_interval {
        fixed
    } else {
        get_block_delay_from_constants(constants)
    };

    let mut block_level = config.sync_block_level;

    while config.enable_sync {
        let block = rpc
            .get_block()
            .block_id(&block::BlockId::Level(block_level))
            .send()
            .await?;

        println!("Syncing the block: {}", block.header.level);
        let hash_string = String::from(block.hash.clone());
        let block_records = (
            blocks::id.eq(block.header.level),
            blocks::hash.eq(hash_string.clone()),
            blocks::timestamp.eq(block.header.timestamp.to_string()),
        );
        diesel::insert_or_ignore_into(blocks::table)
            .values(block_records.clone())
            .on_conflict(blocks::id)
            .do_update()
            .set(block_records)
            .execute(conn)?;

        let events = get_all_events(&block, config.clone()).await?;
        println!("With {} events", events.len());

        for event in events {
            let r#type = serde_json::to_string(&event.r#type).unwrap();
            let payload = serde_json::to_string(&event.payload).unwrap();
            let status = status_to_string(event.result.status);
            let source = String::from(event.source);
            // let event_hash = calculate_unique_id(block.header.level as i128, event.nonce as i128);
            let event_model = EventModel::new(
                // event_hash,
                source.clone(),
                event.r#type,
                event.tag.clone(),
                event.nonce,
                event.payload,
                Some(String::from(status)),
                event.result.consumed_milligas.clone(),
                block.header.level,
            );

            let event_records = (
                // events::id.eq(event_hash as i32),
                events::source.eq(source),
                events::nonce.eq(event.nonce as i32),
                events::type_.eq(r#type),
                events::tag.eq(event.tag.clone()),
                events::payload.eq(payload),
                events::operation_result_status.eq(status),
                events::operation_result_consumed_milligas.eq(event.result.consumed_milligas),
                events::block_id.eq(block.header.level),
            );
            diesel::insert_into(events::table)
                .values(event_records.clone())
                .on_conflict((events::nonce, events::block_id))
                .do_update()
                .set(event_records)
                .execute(conn)?;

            app_state
                .tx
                .send(serde_json::to_string(&event_model).unwrap())
                .unwrap();
        }

        if config.sync_block_level > 0 {
            block_level += 1;
        }

        // sleep some seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(pooling_interval)).await;
    }
    Ok(())
}

async fn get_all_events(
    block: &Block,
    config: Arc<Config>,
) -> Result<Vec<InternalEventOperationResult>, Box<dyn Error>> {
    let mut events = Vec::new();
    let Some(operations) = block.operations.get(3) else { return Ok(events) };

    let contents = operations.iter().flat_map(|op| &op.contents);

    for content in contents {
        match content {
            OperationContent::Transaction(transaction) => match &transaction.metadata {
                Some(metadata) => {
                    metadata
                        .internal_operation_results
                        .iter()
                        .for_each(|internal_operation| match internal_operation {
                            InternalOperationResult::Event(event) => {
                                if let SmartContracts::Only(smart_contracts) =
                                    &config.clone().smart_contracts
                                {
                                    if smart_contracts.contains(&String::from(event.source.clone()))
                                    {
                                        events.push(event.clone());
                                    }
                                } else {
                                    events.push(event.clone());
                                }
                            }
                            _ => return,
                        })
                }
                None => (),
            },
            _ => (),
        }
    }
    Ok(events)
}

fn status_to_string(status: OperationResultStatus) -> &'static str {
    match status {
        OperationResultStatus::Applied => "Applied",
        OperationResultStatus::Skipped => "Skipped",
        OperationResultStatus::Backtracked => "Backtracked",
        OperationResultStatus::Failed => "Failed",
    }
}

fn get_block_delay_from_constants(constants: Constants) -> u64 {
    constants
        .minimal_block_delay
        .unwrap_or_else(|| DEFAULT_BLOCK_TIME) as u64
}
