#!/usr/bin/env bash

source .env
scp -r ./config $DEPLOY_TO
scp -r ./frontend/dist $DEPLOY_TO
scp -r ./backend/target/release/serve $DEPLOY_TO
scp -r ./backend/target/release/import $DEPLOY_TO
scp -r ./.env.example $DEPLOY_TO
