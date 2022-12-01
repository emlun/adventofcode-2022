#!/bin/sh

YEAR=2022
if [[ -z "$1" ]]; then
  DAY=$(date '+%-d')
  DAY_0=$(date '+%d')
else
  DAY=$(date --date="$YEAR-12-$1" '+%-d')
  DAY_0=$(date --date="$YEAR-12-$1" '+%d')
fi

TRIGGER_TIMESTAMP="${YEAR}-12-${DAY_0}T05:00:00+00:00"
check_time() {
  [[ "$(date -Is -u)" < "${TRIGGER_TIMESTAMP}" ]]
}
if check_time; then
  echo "waiting until $TRIGGER_TIMESTAMP"
  while check_time; do sleep 1; done
fi

curl --cookie "session=${SESSION_COOKIE}" "https://adventofcode.com/${YEAR}/day/${DAY}/input" | tee "inputs/day${DAY_0}.in"
