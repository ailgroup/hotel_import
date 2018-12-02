CREATE TABLE register_downloads
(
    started_at timestamp with time zone NOT NULL,
    finished_at timestamp with time zone,
    success bool,
    status_msg text,
    download_time_seconds bigserial,
    download_target_file varchar(100),
    supply_origin_url varchar(250),
    supplier varchar(100)
);

CREATE INDEX ON register_downloads(supplier);
CREATE INDEX ON register_downloads(started_at);
CREATE INDEX ON register_downloads(success);
CREATE INDEX ON register_downloads(finished_at);