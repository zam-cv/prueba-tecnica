#!/bin/bash

# if any command fails, the script will stop running
set -e

# Start the SSH daemon
service ssh start

# Wait for the database to be ready
dockerize -wait tcp://${DATABASE_HOST}:5432 -timeout 180s

# Apply the database migrations
diesel setup
diesel migration run

# Run the main container command
exec "$@"
