DOCKER_COMPOSE_BIN=docker compose
DOCKER_BIN=docker
PROJECT_NAME=sonarcute
IMAGE_VERSION=latest
DOCKER_NETWORK_NAME=sonarcute-network

.DEFAULT_GOAL := help

.PHONY: help build-api-image build-web-image build-token-generator-image create-network gen-token app-base-setup setup clean down clean-setup

help: ## Show this help message
	@echo "Available commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo ""

build-api-image: ## Build the API Docker image
	$(DOCKER_BIN) build -t ${PROJECT_NAME}-api:$(IMAGE_VERSION) -f ./deploy/dockerfile/api.Dockerfile ./api

build-web-image: ## Build the web Docker image
	$(DOCKER_BIN) build -t ${PROJECT_NAME}-web:$(IMAGE_VERSION) -f ./deploy/dockerfile/web.Dockerfile ./web

build-token-generator-image: ## Build the token generator Docker image
	$(DOCKER_BIN) build -t ${PROJECT_NAME}-token-generator:$(IMAGE_VERSION) -f ./deploy/dockerfile/token.Dockerfile ./deploy

create-network: ## Create Docker network if it doesn't exist
	 $(DOCKER_BIN) network inspect $(DOCKER_NETWORK_NAME) >/dev/null 2>&1 || $(DOCKER_BIN) network create $(DOCKER_NETWORK_NAME)

gen-token: ## Generate admin tokens using token generator service
	$(DOCKER_COMPOSE_BIN) -f ./deploy/compose/docker-base-compose.yml -f ./deploy/compose/docker-app-compose.yml -p sonarcute --profile init run --rm tokengenerator

app-base-setup: ## Start all base services (database, SonarQube, etc.) in detached mode
	$(DOCKER_COMPOSE_BIN) -f ./deploy/compose/docker-base-compose.yml -f ./deploy/compose/docker-app-compose.yml -p sonarcute up -d

clean: ## Stop and remove containers, networks, and volumes
	$(DOCKER_COMPOSE_BIN) -f ./deploy/compose/docker-base-compose.yml -f ./deploy/compose/docker-app-compose.yml -p sonarcute down -v || true

down: clean ## Alias for clean command

setup: down build-web-image build-api-image build-token-generator-image create-network app-base-setup gen-token ## Clean all old compose then build all images, create network, start services, and generate tokens

clean-setup: clean setup ## Clean old containers/volumes and then setup everything
