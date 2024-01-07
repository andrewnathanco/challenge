FROM node:20-slim AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /app
WORKDIR /app

FROM base AS build

ARG VERSION
ENV VITE_VERSION=${VERSION}

ARG BASE_URL
ENV VITE_BASE_URL=${BASE_URL}

ARG MURAL_DB
ENV VITE_DB_URL=${MURAL_DB}

ARG SHARE_URL
ENV VITE_SHARE_URL=${SHARE_URL}

RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build

FROM base
COPY --from=build /app/node_modules /app/node_modules
COPY --from=build /app/dist /app/dist
CMD [ "pnpm", "start" ]