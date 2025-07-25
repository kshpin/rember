#!/bin/bash

echo "Testing websocket connection to localhost:3210 in interactive mode."
echo "Type your message and press enter to send."

websocat -t ws://localhost:3210
