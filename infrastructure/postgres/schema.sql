-- PostgreSQL Metadata Schema for the zkVM Scheduler & Verifier

CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    api_key_hash VARCHAR(64) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    plan_type VARCHAR(50) DEFAULT 'free',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS jobs (
    job_id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES tenants(id),
    status VARCHAR(50) NOT NULL, -- QUEUED, PROCESSING, COMPLETED, FAILED
    trace_id VARCHAR(255) NOT NULL,
    proof_id VARCHAR(255),
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP WITH TIME ZONE,
    latency_ms BIGINT
);

CREATE INDEX idx_jobs_tenant ON jobs(tenant_id);
CREATE INDEX idx_jobs_status ON jobs(status);

-- Used for rate limiting & billing
CREATE TABLE IF NOT EXISTS billing_metrics (
    tenant_id UUID REFERENCES tenants(id),
    month_year VARCHAR(7) NOT NULL, -- e.g. "2026-03"
    proofs_generated BIGINT DEFAULT 0,
    proofs_verified BIGINT DEFAULT 0,
    PRIMARY KEY (tenant_id, month_year)
);
