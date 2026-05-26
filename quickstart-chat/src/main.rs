mod module_bindings;
use module_bindings::*;
use spacetimedb_sdk::{credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey};

fn main() {
    // Connect to the database
    let ctx = connect_to_db();

    // Register callbacks to run in response to database events.
    register_callbacks(&ctx);

    // Subscribe to queries in order to construct a local partial replica of the database.
    subscribe_to_tables(&ctx);

    // Spawn a thread, where the connection will process messages and invoke callbacks.
    ctx.run_threaded();

    // Handle CLI input
    user_input_loop(&ctx);
}
