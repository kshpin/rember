#!/bin/bash

echo "Testing websocket connection to localhost:3210 in interactive mode."
echo "Type your message and press enter to send."

echo "Try these:"
echo '{"type": "create_note", "data": {"text": "Hello, world!"}}'
echo '{"type": "get_notes", "data": {}}'

echo ""

websocat -t ws://localhost:3210
