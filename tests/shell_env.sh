#!/usr/bin/env bash

export AWS_ACCESS_KEY_ID=minioadmin
export AWS_SECRET_ACCESS_KEY=minioadmin
export AWS_EC2_METADATA_DISABLED=true

export MYSQL_DATABASE=${MYSQL_DATABASE:="default"}
export QUERY_MYSQL_HANDLER_HOST=${QUERY_MYSQL_HANDLER_HOST:="127.0.0.1"}
export QUERY_MYSQL_HANDLER_PORT=${QUERY_MYSQL_HANDLER_PORT:="3307"}
export QUERY_HTTP_HANDLER_PORT=${QUERY_HTTP_HANDLER_PORT:="8000"}
export QUERY_CLICKHOUSE_HTTP_HANDLER_PORT=${QUERY_CLICKHOUSE_HTTP_HANDLER_PORT:="8124"}
export QUERY_MYSQL_HANDLER_SHARE_1_PORT="13307"
export QUERY_MYSQL_HANDLER_SHARE_2_PORT="23307"

export MYSQL_CLIENT_CONNECT="mysql -uroot --host ${QUERY_MYSQL_HANDLER_HOST} --port ${QUERY_MYSQL_HANDLER_PORT} ${MYSQL_DATABASE} -s"

export MYSQL_CLIENT_SHARE_1_CONNECT="mysql -uroot --host ${QUERY_MYSQL_HANDLER_HOST} --port ${QUERY_MYSQL_HANDLER_SHARE_1_PORT} ${MYSQL_DATABASE} -s"
export MYSQL_CLIENT_SHARE_2_CONNECT="mysql -uroot --host ${QUERY_MYSQL_HANDLER_HOST} --port ${QUERY_MYSQL_HANDLER_SHARE_2_PORT} ${MYSQL_DATABASE} -s"
