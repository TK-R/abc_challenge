#!/bin/sh

TEST_CASE=test
BASE_URL=https://
ATCODER_BASE_URL=.contest.atcoder.jp/tasks/ 

MATCH="$(basename -- "$(dirname "$(pwd)")")"
PROBLEM="$(basename -- "$(pwd)")"
SPACE=_

TEST_URL=$BASE_URL$MATCH$ATCODER_BASE_URL$MATCH$SPACE$PROBLEM

if [ ! -e $TEST_CASE ]; then
	oj dl $TEST_URL
fi

cargo build
oj test -c target/debug/ans
