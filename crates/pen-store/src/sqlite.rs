use anyhow::{Context, Result};
use rusqlite::{Connection, OptionalExtension, params};
use std::path::Path;

use crate::migrate::ensure_metadata_schema;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierGenerationRow {
    pub run_id: String,
    pub step_index: u32,
    pub band_index: u32,
    pub frontier_epoch: u32,
    pub worker_count: u16,
    pub spill_generation: u32,
    pub hot_states: u64,
    pub cold_states: u64,
    pub governor_state: String,
    pub pressure_action: String,
    pub rss_bytes: u64,
}

pub struct MetadataDb {
    connection: Connection,
}

impl MetadataDb {
    pub fn open(path: &Path) -> Result<Self> {
        let connection =
            Connection::open(path).with_context(|| format!("open {}", path.display()))?;
        ensure_metadata_schema(&connection).context("apply metadata migrations")?;
        Ok(Self { connection })
    }

    pub fn record_frontier_generation(&self, row: &FrontierGenerationRow) -> Result<()> {
        self.connection
            .execute(
                "
            INSERT OR REPLACE INTO frontier_generations (
                run_id,
                step_index,
                band_index,
                frontier_epoch,
                worker_count,
                spill_generation,
                hot_states,
                cold_states,
                governor_state,
                pressure_action,
                rss_bytes
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ",
                params![
                    row.run_id,
                    row.step_index,
                    row.band_index,
                    row.frontier_epoch,
                    row.worker_count,
                    row.spill_generation,
                    row.hot_states,
                    row.cold_states,
                    row.governor_state,
                    row.pressure_action,
                    row.rss_bytes,
                ],
            )
            .context("insert frontier generation metadata")?;
        self.connection
            .execute(
                "
            INSERT INTO memory_events (
                run_id,
                frontier_epoch,
                governor_state,
                pressure_action,
                rss_bytes
            ) VALUES (?1, ?2, ?3, ?4, ?5)
            ",
                params![
                    row.run_id,
                    row.frontier_epoch,
                    row.governor_state,
                    row.pressure_action,
                    row.rss_bytes,
                ],
            )
            .context("insert memory event metadata")?;
        Ok(())
    }

    pub fn latest_frontier_generation(
        &self,
        run_id: &str,
        step_index: u32,
        band_index: u32,
    ) -> Result<Option<FrontierGenerationRow>> {
        self.connection
            .query_row(
                "
                SELECT
                    run_id,
                    step_index,
                    band_index,
                    frontier_epoch,
                    worker_count,
                    spill_generation,
                    hot_states,
                    cold_states,
                    governor_state,
                    pressure_action,
                    rss_bytes
                FROM frontier_generations
                WHERE run_id = ?1 AND step_index = ?2 AND band_index = ?3
                ORDER BY frontier_epoch DESC
                LIMIT 1
                ",
                params![run_id, step_index, band_index],
                |row| {
                    Ok(FrontierGenerationRow {
                        run_id: row.get(0)?,
                        step_index: row.get(1)?,
                        band_index: row.get(2)?,
                        frontier_epoch: row.get(3)?,
                        worker_count: row.get(4)?,
                        spill_generation: row.get(5)?,
                        hot_states: row.get(6)?,
                        cold_states: row.get(7)?,
                        governor_state: row.get(8)?,
                        pressure_action: row.get(9)?,
                        rss_bytes: row.get(10)?,
                    })
                },
            )
            .optional()
            .context("query frontier generation metadata")
    }
}

#[cfg(test)]
mod tests {
    use super::{FrontierGenerationRow, MetadataDb};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-store-sqlite-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    #[test]
    fn metadata_db_records_and_reads_frontier_generations() {
        let root = temp_dir("metadata");
        let db = MetadataDb::open(&root.join("meta.sqlite3")).expect("open metadata db");
        let row = FrontierGenerationRow {
            run_id: "run-1".to_owned(),
            step_index: 15,
            band_index: 8,
            frontier_epoch: 12,
            worker_count: 3,
            spill_generation: 12,
            hot_states: 2,
            cold_states: 5,
            governor_state: "orange".to_owned(),
            pressure_action: "spill_cold".to_owned(),
            rss_bytes: 512,
        };
        db.record_frontier_generation(&row)
            .expect("record frontier generation");

        let round_trip = db
            .latest_frontier_generation("run-1", 15, 8)
            .expect("query frontier generation")
            .expect("metadata row should exist");
        assert_eq!(round_trip, row);

        fs::remove_dir_all(root).ok();
    }
}
