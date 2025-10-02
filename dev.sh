#!/usr/bin/env bash

cd client; npm run dev -- --host &
cd ../server; cargo run &
cd ../admin; npx serve &

wait
