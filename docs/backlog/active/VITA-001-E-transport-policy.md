# VITA-001-E transport policy residual

Status: Open. Owner: VITA-001-E. Discovered during I010 independent review.

Pinned Talos provider constructs a default reqwest client and exposes no public arbitrary-provider client injection. Initial HTTPS URL validation does not prevent 307/308 POST-body redirects to another origin or HTTP. Reqwest removes Authorization on host/port/scheme change, but conversation/tool data may still be forwarded. This is not strict endpoint confinement.

Required before production/security acceptance: upstream public transport policy supporting no redirects and HTTPS-only; pin reviewed Talos revision, test same-origin/cross-origin/downgrade redirects with synthetic credentials and tool payloads.

I010 is limited to operator-selected trusted provider and non-sensitive VM demonstration prompts/system facts. This residual remains an open substantive security/release gate; local demo success does not resolve it. User networking is outbound-capable including host/LAN destinations, not a network allowlist.
