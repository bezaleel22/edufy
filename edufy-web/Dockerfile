#############################################
# App Builder: From development to production
#############################################
FROM oven/bun:1.2.21-alpine AS builder
WORKDIR /app

# BUILD WEBSITE
COPY ./package*.json ./bun.lockb ./
RUN bun install

COPY . .
RUN bun run build
RUN bun install --production

############################################
# Website: Production image website
############################################
FROM oven/bun:1.2.21-alpine AS website

WORKDIR /app

COPY --from=builder /app/node_modules node_modules
COPY --from=builder /app/package.json package.json
COPY --from=builder /app/build build

ENV NODE_ENV=production
EXPOSE 3000
ENTRYPOINT ["bun", "run", "build/index.js"]