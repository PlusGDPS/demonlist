-- Your SQL goes here
-- Check if the columns exist before attempting to drop them
DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'demons' AND column_name = 'notes') THEN
        ALTER TABLE demons DROP COLUMN notes;
    END IF;
    
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'demons' AND column_name = 'description') THEN
        ALTER TABLE demons DROP COLUMN description;
    END IF;
    
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'members' AND column_name = 'password_salt') THEN
        ALTER TABLE members DROP COLUMN password_salt;
    END IF;
    
    -- Change column types if necessary
    ALTER TABLE members ALTER COLUMN display_name TYPE TEXT;
    ALTER TABLE members ALTER COLUMN name TYPE TEXT;
END $$;

