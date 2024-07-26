ALTER TABLE investments DROP CONSTRAINT IF EXISTS investments_group_id_fkey;
ALTER TABLE investments DROP CONSTRAINT IF EXISTS investments_creator_id_fkey;
ALTER TABLE investments DROP CONSTRAINT IF EXISTS investments_investment_type_id_fkey;

ALTER TABLE investment_groups DROP CONSTRAINT IF EXISTS investment_groups_owner_id_fkey;

DROP TABLE IF EXISTS investments;
DROP TABLE IF EXISTS investment_groups;
DROP TABLE IF EXISTS investment_types;
DROP TABLE IF EXISTS investment_users;
