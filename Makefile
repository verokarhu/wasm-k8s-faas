help:
	@echo -e "Run 'make deploy' to provision cluster.\n\nRun 'make dashboard' to open k9s dashboard."

bench: update # measures how long it takes for the service to start
	@scripts/bench

dashboard: update # opens k9s dashboard into cluster
	@scripts/dashboard

deploy: # provisions and configures cluster
	@scripts/deploy

update: # updates repo on host to latest version
	@scripts/update
