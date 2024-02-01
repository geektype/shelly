AUTH="Authorization:Bearer ${GITHUB_TOKEN}"

curl -L \
    -X POST \
    -H "${AUTH}" \
    -H "Content-Type: application/octet-stream" \
    "https://uploads.github.com/repos/${OWNER}/${REPO}/releases/${RELEASE}/assets?name=shelly-${PLATFORM}" \
    --data-binary "@./target/release/shelly"
