-- Add migration script here
CREATE TABLE IF NOT EXISTS tbl_users
(
    id                    uuid primary key      default uuid_generate_v7(),
    tenant_id             uuid         not null references tbl_tenants (id) on delete cascade,
    --Authentication
    email                 varchar(255) not null,
    password_hash         text         not null,
    email_verified        boolean      not null default false,
    email_verified_at     timestamptz,
    -- Profile
    full_name             varchar(255),
    avatar_url            text,
    phone                 varchar(50),
    -- Rbac
    role                  varchar(50)  not null default 'user',
    -- Status
    status                varchar(50)  not null default 'active',
    last_login_at         timestamptz,
    -- security
    failed_login_attempts integer      not null default 0,
    locked_util           timestamptz,
    password_changed_at   timestamptz,
    -- Audit
    deleted_at            timestamptz,
    created_at            timestamptz  not null default now(),
    created_by            uuid,
    updated_at            timestamptz  not null default now(),
    updated_by            uuid,
    -- Constraints
    constraint users_email_tenant_unique unique (tenant_id, email),
    constraint users_role_check check ( role in ('supper_admin', 'admin', 'manager', 'user', 'viewer')),
    constraint users_status_check check ( status in ('active', 'inactive', 'suspended') )
);

create index idx_users_tenants on tbl_users (tenant_id) where deleted_at is null;
create index idx_users_email on tbl_users (tenant_id, email) where deleted_at is null;
create index idx_users_role on tbl_users (tenant_id, role) where deleted_at is null;
create index idx_users_status on tbl_users (tenant_id, status) where deleted_at is null;
create index idx_users_active on tbl_users (tenant_id, id) where deleted_at is null and status = 'active';

create trigger update_users_update_at
    before update
    on tbl_users
    for each row
execute function update_updated_at_column()
