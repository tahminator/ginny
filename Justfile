run *args:
    cargo run {{ args }}

new-ddl version description *args:
    @clean_v=$(echo "{{ version }}" | sed 's/[^a-zA-Z0-9]/_/g'); \
    clean_d=$(echo "{{ description }}" | sed 's/[^a-zA-Z0-9]/_/g'); \
    cd db/ && sqitch add "V${clean_v}_${clean_d}" -n "{{ description }}" {{ args }}

deploy *args:
    cd db/ && dotenvx run -f ../.env -- sqitch deploy {{ args }}

verify *args:
    cd db/ && dotenvx run -f ../.env -- sqitch verify {{ args }}

revert *args:
    cd db/ && dotenvx run -f ../.env -- sqitch revert {{ args }}
