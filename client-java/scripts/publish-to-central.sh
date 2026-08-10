#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export GPG_TTY="${GPG_TTY:-$(tty 2>/dev/null || true)}"
mvn -Prelease clean deploy -DskipTests -pl '!orbien-spring-boot-demo'
