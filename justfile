ensure-local-buildings-exists msoa:
  #!/usr/bin/env sh
  if [ -f ./data/msoa-local-buildings/{{msoa}}.geojson ]; then
    echo "msoa-local-buildings/{{msoa}}.geojson exists"
  else
   cargo run --bin generate_msoa_local_buildings -- -m {{msoa}}
  fi
  
ensure-usable-exists msoa:
  #!/usr/bin/env sh
  if [ -f ./data/msoa-usable/{{msoa}}.geojson ]; then
    echo "msoa-usable/{{msoa}}.geojson exists"
  else
   cargo run --bin generate_usable_msoa -- -m {{msoa}}
  fi

ensure-stats-exist msoa:
  #!/usr/bin/env sh
  if [ -f ./data/stats/{{msoa}}.json ]; then
    echo "File exists."
  else
   just ensure-local-buildings-exists {{msoa}}
   just ensure-usable-exists {{msoa}}
   cargo run --bin generate_stats -- -m {{msoa}}
  fi
