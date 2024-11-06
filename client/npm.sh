docker run --rm -it -w /app/client -v $(pwd):/app node:16-alpine npm install && npm run build
rm -r wwwroot/*
mv dist/client/* wwwroot/*