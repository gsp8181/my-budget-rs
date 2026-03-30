rm -r wwwroot/*
cd nextgen-client
npm run build
cd ..
cp nextgen-client/build/* wwwroot/
