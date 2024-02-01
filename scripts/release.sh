echo $GITHUB_TOKEN

URL="https://api.github.com/repos/${OWNER}/${REPO}/releases/${RELEASE}"
HEADERS="Authorization:Bearer ${GITHUB_TOKEN}"
DATA="{\"body\": \"yeetus bafeetus\"}"

curl -s -X PATCH -H "${HEADERS}" -H "Content-Type: application/json" -d "${DATA}" "${URL}"