#!/bin/bash
# Move #[cfg(test)] mod tests { ... } from middle of the file to the end
awk '
/^\#\[cfg\(test\)\]/ { in_test=1; test_block=$0"\n"; next }
in_test {
    test_block = test_block $0 "\n"
    if ($0 == "}") in_test=0
    next
}
{ print }
END { printf "\n%s", test_block }
' crates/screenpipe-core/src/language.rs > crates/screenpipe-core/src/language.rs.tmp
mv crates/screenpipe-core/src/language.rs.tmp crates/screenpipe-core/src/language.rs
