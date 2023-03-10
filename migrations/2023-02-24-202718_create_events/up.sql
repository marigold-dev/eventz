-- Your SQL goes here
CREATE TABLE blocks (
  id INTEGER PRIMARY KEY NOT NULL,
  hash VARCHAR NOT NULL,
  [timestamp] TEXT NOT NULL
);

CREATE TABLE events (
  /* id INTEGER PRIMARY KEY NOT NULL, */
  /* [timestamp] TEXT NOT NULL, */
  source VARCHAR NOT NULL,
  tag VARCHAR NOT NULL,
  nonce INTEGER NOT NULL,
  [type] TEXT NOT NULL,
  payload TEXT NOT NULL,
  operation_result_status VARCHAR NULL,
  operation_result_consumed_milligas VARCHAR NULL,
  block_id INTEGER NOT NULL,
  PRIMARY KEY (nonce, block_id),
  FOREIGN KEY (block_id) REFERENCES blocks (block_id)
);
