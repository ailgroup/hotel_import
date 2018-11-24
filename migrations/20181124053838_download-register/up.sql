CREATE TABLE download_registers
(
    started_at timestamp(6) NOT NULL,
    finished_at timestamp(6),
    download_time_seconds float4,
    download_target_file text,
    supply_origin_url text,
    supplier text
);

CREATE INDEX ON download_registers(supplier);
CREATE INDEX ON download_registers(started_at);
CREATE INDEX ON download_registers(finished_at);