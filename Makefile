#!make
ENV_FILE = .env

# to share env here
include $(ENV_FILE)

PG_DSN = postgres://$(SWD_PG_USER):$(SWD_PG_PASSWORD)@localhost:$(SWD_PG_PORT)/$(SWD_PG_DBNAME)?sslmode=disable
# to share env in program
export $(shell sed 's/=.*//' $(ENV_FILE))
export DB_URI=$(PG_DSN)

migrate:
	refinery migrate -e DB_URI -p ./api/migrations