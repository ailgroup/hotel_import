CREATE TABLE download_registers
(
    started_at timestamp with time zone NOT NULL,
    finished_at timestamp with time zone,
    download_time_seconds bigserial,
    download_target_file varchar(100),
    supply_origin_url varchar(250),
    supplier varchar(100)
);

CREATE INDEX ON download_registers(supplier);
CREATE INDEX ON download_registers(started_at);
CREATE INDEX ON download_registers(finished_at);