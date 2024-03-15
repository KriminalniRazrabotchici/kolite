#!/bin/bash


DB_NAME="test"
DB_HOST="localhost"
MONGO_PORT="27017"

# Drop the database
mongosh $DB_HOST:$MONGO_PORT/$DB_NAME --eval "db.dropDatabase()"
