#!/usr/bin/env bash

cd frontend; npm run dev -- --host --port 4420 &
cd ../backend; cargo run &
cd ../admin; npx serve -l 4422 &

wait
