.PHONY: build
build:
	@source .env && docker-compose build

.PHONY: run
run:
	@docker-compose up
