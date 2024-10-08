#!/bin/bash

export MONGO_DATA=$(pwd)/backend/dataBase

docker-compose up --build -d
