ingest:
	./app server

worker:
	./app worker

build:
	docker build -t arinono/wuxia2kindle .
