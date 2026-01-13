#!/bin/bash

if [ -f "client" ]; then
    echo "Client binary already exists, skipping download."
else
    echo "Downloading Poker Planning Client..."
    curl -L -o client https://github.com/nicoalimin/poker-planning-cli/releases/download/v1.0.0/client
fi

echo "Making executable..."
chmod +x client

echo "Done! Run ./client to start."
