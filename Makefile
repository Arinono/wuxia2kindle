web:
	deno task -c client/deno.json start

ingest:
	./app ingest

worker:
	./app worker
