-- Add migration script here
create table tbl_tenants
(
    id              uuid primary key      default uuid_generate_v7(),
    name            varchar(255) not null,
    slug            varchar(100) not null unique,
    -- plan
    plan            varchar(50)  not null default 'free',
    plan_expires_at timestamptz,
    -- Setting configuration
    setting         jsonb        not null default '{}',
    -- Status
    status          varchar(50)  not null default 'active',
    -- Audit
    created_at      timestamptz  not null default now(),
    created_by      uuid,
    updated_at      timestamptz  not null default now(),
    updated_by      uuid,
    -- Constraints
    constraint tenants_plan_check check (plan IN ('free', 'starter', 'professional', 'enterprise')),
    constraint tenants_status_check check (status IN ('active', 'suspended', 'cancelled'))
);

create index idx_tenants_slug on tbl_tenants (slug) where deleted_at is null;
create index idx_tenants_status on tbl_tenants (status) where deleted_at is null;
create index idx_tenants_active on tbl_tenants (id) where deleted_at is null and status = 'active';

create trigger update_tenants_update_at
    before update
    on tbl_tenants
    for each row
execute function update_updated_at_column();
