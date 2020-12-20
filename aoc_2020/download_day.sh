echo "https://adventofcode.com/2020/day/$1/input"
curl "https://adventofcode.com/2020/day/$1/input" \
-H 'authority: adventofcode.com' \
-H 'pragma: no-cache' \
-H 'cache-control: no-cache' \
-H 'upgrade-insecure-requests: 1' \
-H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.120 Safari/537.36' \
-H 'sec-fetch-mode: navigate' \
-H 'sec-fetch-user: ?1' \
-H 'accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3' \
-H 'sec-fetch-site: same-origin' \
-H "referer: https://adventofcode.com/2020/day/$1" \
-H 'accept-encoding: gzip, deflate, br' \
-H 'accept-language: en-GB,en-US;q=0.9,en;q=0.8' \
-H 'cookie: session=53616c7465645f5f8dea44b560050f5c2433fae4014bcb5c20ba6b87526a5cd693252e35fdef4fc82d98d0be6174d84b' \
--compressed > src/day${1}.txt