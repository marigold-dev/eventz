use std::sync::{Arc};

use crate::AppState;

use {
    crate::db::{self, schema::*},
    diesel::prelude::*,
    std::error::Error,
    tezos_rpc::{
        client::TezosRpc,
        models::{
            block,
            block::Block,
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

async fn get_all_events(
    block: &Block,
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
                                events.push(event.clone());
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

pub async fn run(app_state: Arc<AppState>) -> Result<(), Box<dyn Error>> {
    let conn = &mut db::connect::establish_connection();
    let rpc = TezosRpc::new("https://mainnet.tezos.marigold.dev".into());
    let constants = rpc
        .get_constants()
        .block_id(&block::BlockId::Head)
        .send()
        .await;

    let minimal_block_delay = match constants {
        Ok(constants) => constants
            .minimal_block_delay
            .unwrap_or_else(|| DEFAULT_BLOCK_TIME) as u64,
        Err(e) => {
            println!(
                "Could not get constants, using default minimal_block_delay, Error: {}",
                e
            );
            DEFAULT_BLOCK_TIME as u64
        }
    };

    while 1 != 2 {
        let block = rpc
            .get_block()
            // .block_id(&block::BlockId::Level(-1))
            .block_id(&block::BlockId::Level(3174814))
            .send()
            .await?;
        println!("Syncing the block: {}", block.header.level);
        let hash_string = String::from(block.hash.clone());
        diesel::insert_or_ignore_into(blocks::table)
            .values((
                blocks::id.eq(block.header.level),
                blocks::hash.eq(hash_string),
            ))
            .execute(conn)?;

        let events = get_all_events(&block).await?;
        println!("With {} events", events.len());

        for event in events {
            let r#type = serde_json::to_string(&event.r#type).unwrap();
            let payload = serde_json::to_string(&event.payload).unwrap();
            let status = status_to_string(event.result.status);
            let source = String::from(event.source);
            diesel::insert_into(events::table)
                .values((
                    events::source.eq(source),
                    events::nonce.eq(event.nonce as i32),
                    events::type_.eq(r#type),
                    events::tag.eq(event.tag.clone()),
                    events::payload.eq(payload),
                    events::operation_result_status.eq(status),
                    events::operation_result_consumed_milligas.eq(event.result.consumed_milligas),
                    events::block_id.eq(block.header.level),
                ))
                .execute(conn)?;
            app_state.tx.send(String::from(event.tag)).unwrap();
        }

        // sleep some seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(minimal_block_delay)).await;
    }
    Ok(())
}
