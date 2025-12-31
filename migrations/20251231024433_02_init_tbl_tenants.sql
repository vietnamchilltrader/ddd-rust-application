-- Add migration script here
CREATE TABLE tbl_tenants (
    tenant_id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    plan VARCHAR(50) NOT NULL DEFAULT 'free',
    plan_expires_at TIMESTAMPTZ,
    settings JSONB DEFAULT '{}',
    status VARCHAR(50) DEFAULT 'active',
    deleted_at TIMESTAMPTZ,--soft delete column
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT tenants_plan_check CHECK ( plan IN ('free','starter','professional','enterprise')),
    CONSTRAINT tenants_status_check CHECK ( status IN ('active','suspended','cancelled'))
);
--indexes
CREATE INDEX idx_tenants_slug ON tbl_tenants(slug) WHERE deleted_at IS NULL;
CREATE INDEX idx_tenants_status ON tbl_tenants(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_tenants_active ON tbl_tenants(tenant_id) WHERE deleted_at IS NULL AND status = 'active';

