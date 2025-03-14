FROM node:latest

WORKDIR /app
COPY ./package.json /app/
COPY ./package-lock.json /app/
RUN npm install

VOLUME /app
COPY index.html tsconfig.app.json tsconfig.json tsconfig.node.json vite.config.ts /app
COPY ./public /app/public 
COPY ./src /app/src

EXPOSE 5173
