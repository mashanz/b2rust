FROM oven/bun:latest AS develop
WORKDIR /app
COPY . .
RUN bun install
CMD ["bun", "run", "tw:watch"]
