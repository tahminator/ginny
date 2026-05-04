-- Deploy ginny:V0001_Creating_test_table to pg

BEGIN;

CREATE TABLE test (
  id INT primary key GENERATED ALWAYS AS IDENTITY,
  description TEXT NULL
);

COMMIT;
