use actix::System;
use futures::{future, FutureExt};

use hkt_actix_test_utils::run_actix;
use hkt_jsonrpc::client::new_http_client;
use hkt_o11y::testonly::init_test_logger;

use hkt_jsonrpc_tests as test_utils;

/// Retrieve client status via HTTP GET.
#[test]
fn test_status() {
    init_test_logger();

    run_actix(async {
        let (_view_client_addr, addr) = test_utils::start_all(test_utils::NodeType::NonValidator);

        let client = new_http_client(&format!("http://{}", addr));
        actix::spawn(client.status().then(|res| {
            let res = res.unwrap();
            assert_eq!(res.chain_id, "unittest");
            assert_eq!(res.sync_info.latest_block_height, 0);
            assert_eq!(res.sync_info.syncing, false);
            System::current().stop();
            future::ready(())
        }));
    });
}
