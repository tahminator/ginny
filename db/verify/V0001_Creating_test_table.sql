-- Verify ginny:V0001_Creating_test_table on pg

DO $$
BEGIN

ASSERT (
    SELECT 1 FROM pg_tables
    WHERE schemaname = 'public'
    AND tablename = 'test'
);

END $$;
