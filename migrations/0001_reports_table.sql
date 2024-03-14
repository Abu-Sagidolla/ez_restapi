CREATE SEQUENCE scan_counter;

CREATE TABLE Scanreport (
    id SERIAL PRIMARY KEY,
    scandata JSONB,
    scanned TIMESTAMP,
    scan_counter INTEGER DEFAULT nextval('scan_counter')
);