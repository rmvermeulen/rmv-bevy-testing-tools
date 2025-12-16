test-all *FLAGS:
    cargo fc --silent --fail-fast --pedantic nextest run {{FLAGS}}
