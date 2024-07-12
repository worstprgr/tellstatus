.PHONY: all fast clean docker

all: notify-release build-rel-raspi notify-docker docker-build-raspi docker-save
fast: notify-release build-rel-raspi notify-docker docker-build-raspi docker-save docker-run
fast-linux: notify-docker build-rel-lnx docker-build-linux docker-save docker-run
docker-pi: notify-docker docker-build-raspi docker-save
docker-pi-run: notify-docker docker-build-raspi docker-save docker-run
docker-lnx: notify-docker docker-build-linux docker-save
docker-lnx-run: notify-docker docker-build-linux docker-save docker-run

APPNAME := tellstatus

raspi4 := aarch64-unknown-linux-gnu
windows := x86_64-pc-windows-msvc
linux64 := x86_64-unknown-linux-gnu


notify-release:
	@echo "### Building (Prod)"

notify-docker:
	@echo "### Building, saving and running Docker image ..."

build-current-system:
	@echo "[Cargo] -> Building for current system ..."
	cargo build -r

build-rel-raspi:
	@echo "[Cross] -> $(raspi4) ..."
	cross build --target $(raspi4) -r

build-rel-lnx:
	@echo "[Cross] -> $(linux64) ..."
	cross build --target $(linux64) -r

build-rel-win:
	@echo "[Cross] -> $(windows) ..."
	cross build --target $(windows) -r


docker-build-raspi:
	@echo "[Docker] -> Platform: Linux/ARM64 (Raspi4)"
	docker build -t $(APPNAME) --platform linux/arm64 .

docker-build-linux:
	@echo "[Docker] -> Platform: Linux/amd64"
	docker build -t $(APPNAME) --platform linux/amd64 .

docker-save:
	@echo "[Docker] -> Saving image ..."
	docker save -o $(APPNAME).tar $(APPNAME):latest

docker-run:
	docker run -d --name $(APPNAME) $(APPNAME):latest


clean:
	@echo "Removing Images from Docker ..."
	docker rmi $(APPNAME):latest
