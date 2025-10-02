#!/usr/bin/env bash

cd client; npm run dev -- --host --port 4420 &
cd ../server; cargo run &
cd ../admin; npx serve -l 4422 &

wait
