use near_indexer_primitives::types;
use near_o11y::WithSpanContextExt;

/// Fetches the status to retrieve `latest_block_height` to determine if we need to fetch
/// entire block or we already fetched this block.
pub(crate) async fn fetch_latest_block(
    client: &actix::Addr<near_client::ViewClientActor>,
) -> anyhow::Result<u64> {
    let block = client
        .send(
            near_client::GetBlock(types::BlockReference::Finality(types::Finality::Final))
                .with_span_context(),
        )
        .await??;
    Ok(block.header.height)
}