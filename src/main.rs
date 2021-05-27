use jsonrpsee_ws_client::WsClientBuilder;

#[tokio::main]
async fn main() {
	let _ws_client = WsClientBuilder::default()
		.max_request_body_size(u32::MAX)
		.build("ws://127.0.0.1:9944")
		.await
		.unwrap();
}
