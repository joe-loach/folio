dev:
    npx wrangler dev --local

css:
    npx @tailwindcss/cli -i ./src/input.css -o ./public/tailwind.css --watch

deploy:
    npx wrangler deploy
