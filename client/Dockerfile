FROM node:latest

WORKDIR /app
COPY ./package.json /app/
COPY ./package-lock.json /app/
RUN npm install

VOLUME /app
COPY index.html tsconfig.app.json tsconfig.json tsconfig.node.json vite.config.ts ./public ./src /app/

EXPOSE 5173

CMD npm run dev_dc
