set -o pipefail

if [ -z "$1" ]; then
    newest_hash="$(curl -f -s https://api.github.com/repos/cyl3x/shopware-devenv/commits?per_page=1 | jq -r '.[0].sha')"

    if [ "$?" != "0" ] || [ "$newest_hash" == "" ]; then
        echo "Unable to fetch newest commit sha, maybe rate limted?"
        exit 1
    fi
else
    newest_hash="$1"
fi

echo "Using newest hash $newest_hash"

update_module() {
    echo "Update '$1'"
    sed -Ein "s/(.*rev = \")[a-z0-9]*(\";)/\1$newest_hash\2/" "$1"
}
