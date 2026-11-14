#!/usr/bin/env bash
. ../../catalog.sh

search() {
  catalog_items | grep "$1"
}
