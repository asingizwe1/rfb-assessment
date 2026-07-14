rpc.call("getblockchaininfo", &[])
    .context("Failed to connect to Bitcoin Core — check your RPC URL and credentials")?