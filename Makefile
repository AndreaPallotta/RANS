# Manage arangodb3 on Docker for development and testing

CONTAINER_NAME := arangodb
IMAGE := arangodb/arangodb:latest
ROOT_PASSWORD := root
WEB_PORT := 8529
DATA_DIR := $(PWD)/arangodb3
DOCKER_RUN_CMD := docker run \
  --name $(CONTAINER_NAME) \
  -e ARANGO_ROOT_PASSWORD=$(ROOT_PASSWORD) \
  -p $(WEB_PORT):8529 \
  -v $(DATA_DIR):/var/lib/arangodb3 \
  $(IMAGE)

start:
	@echo Starting $(CONTAINER_NAME) container...
	$(DOCKER_RUN_CMD)

stop:
	@echo Stopping $(CONTAINER_NAME) container...
	docker stop $(CONTAINER_NAME)
	docker rm $(CONTAINER_NAME)

backup:
	@echo Backing up ArangoDB data files...
	docker cp $(CONTAINER_NAME):/var/lib/arangodb3 $(DATA_DIR)/backup-$(shell date +"%Y%m%d%H%M%S")

.PHONY: start stop backup
