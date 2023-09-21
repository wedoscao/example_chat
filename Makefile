tailwind_out = ./public/tailwind.css
tailwind = npx tailwindcss -o $(tailwind_out) --minify

dev: 
	@cargo watch -c -w "./src" -w "./tailwind.config.ts" -s "$(tailwind) && cargo run"

prod: 
	@$(tailwind)
	@cargo run -r
