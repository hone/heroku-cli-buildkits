'use strict'

import HTTP from 'http-call'

export default class BuildpackRegistry {
  static options = {
      method: 'GET',
      protocol: 'https:',
      path: '/',
      raw: false,
      partial: false
  }

  static url() {
    if (process.env.HEROKU_BUILDPACK_REGISTRY_URL === undefined) {
      return 'buildkits2-api-staging.herokuapp.com';
    } else {
      return process.env.HEROKU_BUILDPACK_REGISTRY_URL;
    }
  }

  static headers(token) {
    let defaultHeaders = {
      'Authorization': `Bearer ${token}`,
      'Accept': 'application/vnd.heroku+json; version=3.buildpack-registry',
      'Content-Type': 'application/json'
    };

    if (process.env.HEROKU_HEADERS) {
      return Object.assign({}, defaultHeaders, JSON.parse(process.env.HEROKU_HEADERS))
    } else {
      return defaultHeaders;
    }
  }

  static create(token) {
    return HTTP.create(Object.assign(this.options, {host: this.url()}, {headers: this.headers(token)}))
  }
}
