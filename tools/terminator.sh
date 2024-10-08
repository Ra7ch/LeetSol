export MONGO_DATA=$(pwd)/backend/dataBase

docker compose down --volumes --remove-orphans --rmi all
