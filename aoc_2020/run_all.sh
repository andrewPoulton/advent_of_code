for (( i=1; i<26; ++i)); do
echo "\nrunning day ${i}"
./target/release/aoc_2020 $i
echo "\n"
done
