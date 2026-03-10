#!/bin/bash
TOTAL_CS=$(find src -name "*.cs" | wc -l)
ASYNC_VOID=$(grep -rn "async void" src/ | wc -l)
RESULT_WAIT=$(grep -rn "\.Result[^a-zA-Z]" src/ | grep -v "Test" | wc -l)

echo "Files: $TOTAL_CS"
echo "Async Void: $ASYNC_VOID"
echo "Result/Wait: $RESULT_WAIT"
