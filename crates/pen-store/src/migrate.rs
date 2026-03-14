use rusqlite::Connection;

pub const METADATA_SCHEMA_VERSION: u32 = 1;

pub fn ensure_metadata_schema(connection: &Connection) -> rusqlite::Result<()> {
    connection.execute_batch(
        "
        BEGIN;
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY
        );
        CREATE TABLE IF NOT EXISTS frontier_generations (
            run_id TEXT NOT NULL,
            step_index INTEGER NOT NULL,
            band_index INTEGER NOT NULL,
            frontier_epoch INTEGER NOT NULL,
            worker_count INTEGER NOT NULL,
            spill_generation INTEGER NOT NULL,
            hot_states INTEGER NOT NULL,
            cold_states INTEGER NOT NULL,
            governor_state TEXT NOT NULL,
            pressure_action TEXT NOT NULL,
            rss_bytes INTEGER NOT NULL,
            PRIMARY KEY (run_id, step_index, band_index, frontier_epoch)
        );
        CREATE TABLE IF NOT EXISTS memory_events (
            run_id TEXT NOT NULL,
            frontier_epoch INTEGER NOT NULL,
            governor_state TEXT NOT NULL,
            pressure_action TEXT NOT NULL,
            rss_bytes INTEGER NOT NULL
        );
        INSERT OR IGNORE INTO schema_migrations(version) VALUES (1);
        COMMIT;
        ",
    )
}
