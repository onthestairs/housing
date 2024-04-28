ensure-zoomstack-file-exists:
  #!/usr/bin/env sh
  if [ -f ./data/uk-zoomstack-geopackage/OS_Open_Zoomstack.gpkg ]; then
    echo "uk-zoomstack-geopackage/OS_Open_Zoomstack.gpkg exists"
  else
    wget -O /tmp/uk-zoomstack.zip https://www.data.gov.uk/dataset/ee11adb7-a1f8-4d18-a261-cd5b64973ccd/middle-layer-super-output-areas-december-2021-boundaries-ew-bgc-v2
    unzip /tmp/uk-zoomstack.zip ./data/uk-zoomstack-geopackage/
    rm /tmp/uk-zoomstack.zip
  fi

ensure-census-population-exists:
  #!/usr/bin/env sh
  if [ -f ./data/census/population.csv ]; then
    echo "census/population.csv exists"
  else
   # generated here: https://www.ons.gov.uk/datasets/create
   wget -O ./data/census/population.csv https://static.ons.gov.uk/datasets/9a097213-84a6-47cc-89de-824e56f14995/custom-filtered-2024-04-28T13:13:41Z.csv#get-data
  fi

ensure-census-accomodation-exists:
  #!/usr/bin/env sh
  if [ -f ./data/census/accomodation.csv ]; then
    echo "census/accomodation.csv exists"
  else
   # generated here: https://www.ons.gov.uk/datasets/create
   wget -O ./data/census/accomodation.csv https://static.ons.gov.uk/datasets/e3f2d13f-ba20-4ada-9774-6daf689197e2/custom-filtered-2024-04-28T12:12:52Z.csv#get-data
  fi


ensure-all-msoas-file-exists:
  #!/usr/bin/env sh
  if [ -f ./data/msoa-all/2021.geojson ]; then
    echo "msoa-all/2021.geojson exists"
  else
   # see https://www.data.gov.uk/dataset/ee11adb7-a1f8-4d18-a261-cd5b64973ccd/middle-layer-super-output-areas-december-2021-boundaries-ew-bgc-v2 
   wget -O ./data/msoa-all/2021.geojson https://open-geography-portalx-ons.hub.arcgis.com/api/download/v1/items/ed5c7b7d733d4fd582281f9bfc9f02a2/geojson?layers=0
  fi

ensure-individual-msoas-exist:
  #!/usr/bin/env sh
  if [ "$(ls -A ./data/msoa/)" ]; then
    echo "msoa/ is populated"
  else
   just ensure-all-msoas-file-exists
   cargo run --release --bin generate_individual_msoa_geojsons
  fi

ensure-local-buildings-exists msoa:
  #!/usr/bin/env sh
  if [ -f ./data/msoa-local-buildings/{{msoa}}.geojson ]; then
    echo "msoa-local-buildings/{{msoa}}.geojson exists"
  else
   just ensure-zoomstack-file-exists
   just ensure-individual-msoas-exist
   cargo run --release --bin generate_msoa_local_buildings -- -m {{msoa}}
  fi
  
ensure-usable-exists msoa:
  #!/usr/bin/env sh
  if [ -f ./data/msoa-usable/{{msoa}}.geojson ]; then
    echo "msoa-usable/{{msoa}}.geojson exists"
  else
   just ensure-zoomstack-file-exists
   just ensure-individual-msoas-exist
   cargo run --release --bin generate_usable_msoa -- -m {{msoa}}
  fi

ensure-stats-exist msoa:
  #!/usr/bin/env sh
  if [ -f ./data/stats/{{msoa}}.json ]; then
    echo "stats/{{msoa}}.geojson exists"
  else
   just ensure-local-buildings-exists {{msoa}}
   just ensure-usable-exists {{msoa}}
   cargo run --bin generate_stats -- -m {{msoa}}
  fi
