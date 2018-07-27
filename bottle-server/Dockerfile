FROM node:9

WORKDIR /bottle-server
ADD . /bottle-server

RUN yarn install

EXPOSE 8080

CMD ["node", "index.js"]