
all:
	chmod +x tools/init.sh
	./tools/init.sh

down:
	chmod +x tools/terminator.sh
	./tools/terminator.sh

ps:
	docker ps

pause:
	docker pause $(docker ps -q)

stop:
	chmod +x tools/stop.sh
	./tools/stop.sh

start:
	chmod +x tools/start.sh
	./tools/start.sh
