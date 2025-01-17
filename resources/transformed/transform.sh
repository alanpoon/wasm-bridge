#!/usr/bin/sh
set -e

## Wasi shim

# copy the files
rm -rf /browser
rm -rf preview2-shim/http

cp -r ../original/preview2-shim/browser preview2-shim
cp -r ../original/preview2-shim/http preview2-shim

# change the import path
cd preview2-shim
sed -i -E 's#@bytecodealliance/preview2-shim#../browser/#' http/wasi-http.js

# TODO: Crypto is not defined ...
cp ../random.js browser/random.js

# bundle the files into a single file
esbuild index.js --bundle --outfile=bundled.js

# return the import object from the bundle
tail -r bundled.js | tail +2 | tail -r > bundled_new.js
echo >> bundled_new.js
echo '  return getWasiImports();' >> bundled_new.js
echo '})();' >> bundled_new.js
mv bundled_new.js bundled.js

cd ..
