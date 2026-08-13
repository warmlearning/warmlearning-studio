ALTER TABLE contact_submissions ADD COLUMN learning_stage TEXT NULL;
ALTER TABLE contact_submissions ADD COLUMN concern TEXT NULL;
ALTER TABLE contact_submissions DROP COLUMN course_interest;
