#!/bin/bash

module load sqlite/3.31.1

set -e 

sqlite3 -separator ',' -header ../src/sql/oscar.db "select id, datetime(time, 'localtime') as time, allocated, idle, other, total from cpu;" > ../data/oscar.csv

t=`date`

git add ../data/oscar.csv
git commit -m "update from ${t}"
git push origin main

