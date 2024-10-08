FROM node:18

COPY srcs /app

WORKDIR /app

RUN npm install
 
RUN npm install axios multer dotenv express cors async fs mongoose 

CMD ["node", "app.js"]