FROM node:16-alpine

WORKDIR /app

COPY package.json ./
COPY yarn.lock ./

RUN yarn install

RUN yarn add prismjs

COPY . .

RUN yarn build

ENV PORT=5000

CMD ["yarn", "start"]
