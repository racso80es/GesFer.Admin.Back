#!/bin/bash
echo "=== Checking Architecture ==="
echo "Collections using List<T> instead of IEnumerable<T> in Application:"
grep -rn "List<" src/GesFer.Admin.Back.Application/

echo "Collections using List<T> instead of IEnumerable<T> in Api:"
grep -rn "List<" src/GesFer.Admin.Back.Api/

echo "DTOs in Api controllers:"
grep -rn "record " src/GesFer.Admin.Back.Api/Controllers/
grep -rn "class .*Dto" src/GesFer.Admin.Back.Api/Controllers/

echo "=== Checking Nomenclature ==="
echo "Async methods without Async suffix:"
grep -rn "async Task [a-zA-Z0-9_]*(" src/ | grep -v "Async("

echo "=== Checking Async Stability ==="
echo ".Result usage:"
grep -rn "\.Result" src/ | grep -v "context.Result"

echo ".Wait() usage:"
grep -rn "\.Wait()" src/

