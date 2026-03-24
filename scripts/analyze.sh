#!/bin/bash
echo "Architecture %: "
# Let's run a check on Async Stability
echo "Checking for .Result, .Wait(), async void..."
grep -rni ".Result" src/ | grep -v "AuthorizeSystemOrAdminAttribute" | grep -v "bin" | grep -v "obj"
grep -rni ".Wait(" src/ | grep -v "bin" | grep -v "obj"
grep -rni "async void" src/ | grep -v "bin" | grep -v "obj"

echo "Checking for List<T> instead of IEnumerable<T> in Application/Api..."
grep -rn "List<" src/GesFer.Admin.Back.Application/ | grep -v "using" | grep -v "bin" | grep -v "obj"
grep -rn "List<" src/GesFer.Admin.Back.Api/ | grep -v "using" | grep -v "bin" | grep -v "obj"

echo "Checking for inline DTOs in API..."
grep -rn "record" src/GesFer.Admin.Back.Api/ | grep -v "using" | grep -v "bin" | grep -v "obj"
grep -rn "class" src/GesFer.Admin.Back.Api/Controllers/ | grep -v "Controller" | grep -v "bin" | grep -v "obj"
