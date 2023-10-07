web:
	deno task -c client/deno.json start

ingest:
	./app ingest -p 3000

worker:
	./app worker
