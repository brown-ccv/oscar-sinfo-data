#!/bin/bash

module load sqlite/3.31.1

sqlite3 -header ../src/sql/oscar.db "select id, datetime(time, 'localtime') as time, allocated, idle, other, total from cpu;" > ../data/oscar.csv
