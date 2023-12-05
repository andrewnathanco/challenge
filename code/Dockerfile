FROM node:20-slim AS base


RUN corepack enable
COPY . /app
WORKDIR /app

FROM base AS build

ARG VERSION
ENV VITE_VERSION=${VERSION}

ARG BASE_URL
ENV VITE_BASE_URL=${BASE_URL}

ARG BASE_PATH
ENV BASE_PATH=${BASE_PATH}

RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build

FROM ubuntu
RUN apt-get update
RUN apt-get install nginx -y
COPY --from=build /app/dist /var/www/html/
CMD ["nginx","-g","daemon off;"]

