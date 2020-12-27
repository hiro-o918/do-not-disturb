FROM node:15.5.0-buster-slim

WORKDIR /usr/local/app
RUN apt-get update \
  && apt-get install -y \
  g++ \
  build-essential \
  python

RUN groupadd -g 2000 app \
  && useradd -M -u 2000 -g 2000 app

WORKDIR /usr/local/app/

COPY . .

RUN yarn install \
  && yarn build

RUN chown app:app .nuxt

USER app
CMD ["yarn", "start"]
