FROM node:20-slim AS base

ARG VERSION
ARG BASE_URL
ENV VITE_VERSION=${VERSION}
ENV VITE_BASE_URL=${BASE_URL}

RUN corepack enable
COPY . /app
WORKDIR /app

FROM base AS build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build

FROM ubuntu
RUN apt-get update
RUN apt-get install nginx -y
COPY --from=build /app/dist /var/www/html/
CMD ["nginx","-g","daemon off;"]

