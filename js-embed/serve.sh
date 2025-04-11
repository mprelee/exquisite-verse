#!/bin/bash

# Check if a server is already running
if [ -n "$(lsof -i:8000)" ]; then
    echo "A server is already running on port 8000. Please stop it first."
    exit 1
fi

# Start a simple HTTP server to serve the application
echo "Starting server on http://localhost:8000"
echo "Press Ctrl+C to stop the server"
python3 -m http.server 8000 